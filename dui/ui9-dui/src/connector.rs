use crate::protocol;
use crate::tracers::PeerTracer;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, Duty, ManagedContext, Next, OnEvent, ToRecipient,
};
use crb::core::{Slot, Unique};
use crb::send::{Recipient, Sender};
use crb::superagent::{Fetcher, ManageSubscription, StateEntry, SubscribeExt, Subscription};
use derive_more::{Deref, DerefMut, From, Into};
use futures::stream::StreamExt;
use libp2p::PeerId;
use libp2p::{
    gossipsub, mdns, noise,
    request_response::{self, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, StreamProtocol, Swarm,
};
use std::{
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::select;
use typed_slab::TypedSlab;

#[derive(Deref, DerefMut, From)]
pub struct ConnectorLink {
    address: Address<Connector>,
}

impl ConnectorLink {
    pub fn open_connection(
        &self,
        peer_id: PeerId,
        recipient: impl ToRecipient<protocol::Response>,
    ) -> Fetcher<StateEntry<OpenConnection>> {
        let msg = OpenConnection {
            peer_id,
            recipient: recipient.to_recipient(),
        };
        self.address.subscribe(msg)
    }
}

pub struct Connector {
    swarm: Slot<Swarm<Ui9Behaviour>>,
    peer_tracer: PeerTracer,

    connections: TypedSlab<ConnectionId, Connection>,
    connection_ids: HashMap<Unique<OpenConnection>, ConnectionId>,
}

impl Connector {
    pub fn new() -> Self {
        Self {
            swarm: Slot::empty(),
            peer_tracer: PeerTracer::new(),
            connections: TypedSlab::new(),
            connection_ids: HashMap::new(),
        }
    }
}

#[derive(NetworkBehaviour)]
struct Ui9Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    request_response: request_response::cbor::Behaviour<protocol::Request, protocol::Response>,
}

#[async_trait]
impl Agent for Connector {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }

    async fn event(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        let swarm = self.swarm.get_mut()?;
        select! {
            envelope = ctx.next_envelope() => {
                if let Some(envelope) = envelope {
                    envelope.handle(self, ctx).await?;
                } else {
                    ctx.stop();
                }
            }
            event = swarm.select_next_some() => {
                self.route_swarm_event(event, ctx).await?;
            }
        }
        Ok(())
    }
}

pub struct Initialize;

#[async_trait]
impl Duty<Initialize> for Connector {
    async fn handle(&mut self, _: Initialize, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                let unique_message = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(unique_message)
                    .build()?;

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;

                let request_response = request_response::cbor::Behaviour::new(
                    [(
                        StreamProtocol::new("/ui9-trace/0.0.1"),
                        ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                );

                Ok(Ui9Behaviour {
                    gossipsub,
                    mdns,
                    request_response,
                })
            })?
            .build();

        let topic = gossipsub::IdentTopic::new("ice-nine-ui9");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        self.swarm.fill(swarm)?;
        Ok(Next::events())
    }
}

impl Connector {
    async fn route_swarm_event(
        &mut self,
        event: SwarmEvent<Ui9BehaviourEvent>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        match event {
            SwarmEvent::Behaviour(event) => match event {
                Ui9BehaviourEvent::Mdns(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                Ui9BehaviourEvent::Gossipsub(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                Ui9BehaviourEvent::RequestResponse(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local node is listening on {address}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.peer_tracer.add_peer(peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                self.peer_tracer.del_peer(peer_id);
            }
            other => {
                log::warn!("Not handeled p2p event: {other:?}");
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<mdns::Event> for Connector {
    async fn handle(&mut self, event: mdns::Event, _ctx: &mut Context<Self>) -> Result<()> {
        use mdns::Event::*;
        let swarm = self.swarm.get_mut()?;
        match event {
            Discovered(list) => {
                for (peer_id, _multiaddr) in list {
                    println!("UI9 node connected: {peer_id}");
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                }
            }
            Expired(list) => {
                for (peer_id, _multiaddr) in list {
                    println!("UI9 node disconnected: {peer_id}");
                    swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<gossipsub::Event> for Connector {
    async fn handle(&mut self, event: gossipsub::Event, _ctx: &mut Context<Self>) -> Result<()> {
        use gossipsub::Event::*;
        if let Message {
            propagation_source,
            message_id,
            message,
        } = event
        {
            println!(
                "Got message: '{}' with id: {message_id} from peer: {propagation_source}",
                String::from_utf8_lossy(&message.data),
            );
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<protocol::Event> for Connector {
    async fn handle(&mut self, event: protocol::Event, _ctx: &mut Context<Self>) -> Result<()> {
        use libp2p::request_response::Message;
        use protocol::Event;
        match event {
            Event::Message { message, .. } => match message {
                Message::Request { request, .. } => {
                    log::warn!("Not handeled request event: {request:?}");
                }
                Message::Response { response, .. } => {
                    log::warn!("Not handeled response event: {response:?}");
                }
            },
            other => {
                log::warn!("Not handeled request_reponse event: {other:?}");
            }
        }
        Ok(())
    }
}

pub struct Connection {
    sub: Unique<OpenConnection>,
}

#[derive(From, Into, Clone, Copy)]
struct ConnectionId(usize);

pub struct ConnectionSender {
    peer_id: PeerId,
    id: ConnectionId,
    recipient: Recipient<ForwardRequest>,
}

impl ConnectionSender {
    pub fn send(&self, request: protocol::Request) -> Result<()> {
        let request = ForwardRequest {
            peer_id: self.peer_id,
            id: self.id,
            request,
        };
        self.recipient.send(request)
    }
}

pub struct OpenConnection {
    peer_id: PeerId,
    recipient: Recipient<protocol::Response>,
}

impl Subscription for OpenConnection {
    type State = ConnectionSender;
}

#[async_trait]
impl ManageSubscription<OpenConnection> for Connector {
    async fn subscribe(
        &mut self,
        sub: Unique<OpenConnection>,
        ctx: &mut Context<Self>,
    ) -> Result<ConnectionSender> {
        let connection = Connection { sub: sub.clone() };
        let id = self.connections.insert(connection);
        let connection = ConnectionSender {
            peer_id: sub.peer_id.clone(),
            id,
            recipient: ctx.recipient(),
        };
        self.connection_ids.insert(sub, id);
        Ok(connection)
    }

    async fn unsubscribe(
        &mut self,
        sub: Unique<OpenConnection>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        let id = self.connection_ids.remove(&sub);
        if let Some(id) = id {
            self.connections.remove(id);
        }
        Ok(())
    }
}

struct ForwardRequest {
    peer_id: PeerId,
    id: ConnectionId,
    request: protocol::Request,
}

#[async_trait]
impl OnEvent<ForwardRequest> for Connector {
    async fn handle(&mut self, event: ForwardRequest, _ctx: &mut Context<Self>) -> Result<()> {
        let swarm = self.swarm.get_mut()?.behaviour_mut();
        let _req_id = swarm
            .request_response
            .send_request(&event.peer_id, event.request);
        Ok(())
    }
}
