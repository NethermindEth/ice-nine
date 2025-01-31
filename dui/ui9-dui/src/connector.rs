use crate::protocol::{self, Envelope, Request, Response, SessionId};
use crate::router::Router;
use crate::subscriber::Relay;
use crate::tracers::peer::Peer;
use crate::Pub;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, ManagedContext, Next, OnEvent, ToRecipient};
use crb::core::{Slot, Unique};
use crb::send::{Recipient, Sender};
use crb::superagent::{
    Fetcher, ManageSubscription, StateEntry, SubscribeExt, Subscription, Supervisor,
    SupervisorSession,
};
use derive_more::{Deref, DerefMut, From};
use futures::stream::StreamExt;
use libp2p::PeerId;
use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, StreamProtocol, Swarm,
};
use libp2p_stream as stream;
use std::{
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::select;
use typed_slab::TypedSlab;
use ui9_request_response::{self as request_response, ProtocolSupport, ResponseChannel};

#[derive(Deref, DerefMut, From)]
pub struct ConnectorLink {
    address: Address<Connector>,
}

impl ConnectorLink {
    pub fn open_session(
        &self,
        peer_id: PeerId,
        recipient: impl ToRecipient<Response>,
    ) -> Fetcher<StateEntry<OpenSession>> {
        let msg = OpenSession {
            peer_id,
            recipient: recipient.to_recipient(),
        };
        self.address.subscribe(msg)
    }
}

#[derive(Default)]
struct Outgoing {
    sessions: TypedSlab<SessionId, Session>,
    session_ids: HashMap<Unique<OpenSession>, SessionId>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct SessionKey {
    peer_id: PeerId,
    session_id: SessionId,
}

struct SessionRecord {
    relay: Address<Relay>,
    channel: ResponseChannel<Envelope<Response>>,
}

#[derive(Default)]
struct Incoming {
    relays: HashMap<SessionKey, SessionRecord>,
}

pub struct Connector {
    swarm: Slot<Swarm<Ui9Behaviour>>,
    peer_tracer: Pub<Peer>,
    outgoing: Outgoing,
    incoming: Incoming,
}

impl Connector {
    pub fn new() -> Self {
        Self {
            swarm: Slot::empty(),
            peer_tracer: Pub::unified(),
            outgoing: Outgoing::default(),
            incoming: Incoming::default(),
        }
    }
}

#[derive(NetworkBehaviour)]
struct Ui9Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    request_response: request_response::cbor::Behaviour<Envelope<Request>, Envelope<Response>>,
    stream: stream::Behaviour,
}

impl Supervisor for Connector {
    type GroupBy = ();
}

#[async_trait]
impl Agent for Connector {
    type Context = SupervisorSession<Self>;

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
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
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

                let stream = stream::Behaviour::new();

                Ok(Ui9Behaviour {
                    gossipsub,
                    mdns,
                    request_response,
                    stream,
                })
            })?
            .build();

        let topic = gossipsub::IdentTopic::new("ice-nine-ui9");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        let control = swarm.behaviour_mut().stream.new_control();
        let router = Router::new(control);
        ctx.spawn_agent(router, ());

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
                Ui9BehaviourEvent::Stream(()) => {
                    log::info!("Start streaming");
                }
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                log::info!("Local node is listening on {address}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                log::debug!("Connection to {peer_id} has established");
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                log::debug!("Connection to {peer_id} has closed");
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
                    log::trace!("UI9 node discovered: {peer_id}");
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    self.peer_tracer.add_peer(peer_id);
                }
            }
            Expired(list) => {
                for (peer_id, _multiaddr) in list {
                    log::trace!("UI9 node exipred: {peer_id}");
                    swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                    self.peer_tracer.del_peer(peer_id);
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
            log::trace!(
                "Got message: '{}' with id: {message_id} from peer: {propagation_source}",
                String::from_utf8_lossy(&message.data),
            );
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<protocol::Event> for Connector {
    async fn handle(&mut self, event: protocol::Event, ctx: &mut Context<Self>) -> Result<()> {
        use protocol::Event;
        use ui9_request_response::Message;
        match event {
            Event::Message { message, .. } => match message {
                Message::Request {
                    request,
                    peer,
                    channel,
                    ..
                } => {
                    let session_key = SessionKey {
                        peer_id: peer,
                        session_id: request.session_id,
                    };
                    log::warn!("Not handeled request event: {request:?}");
                    match request.value {
                        Request::Subscribe(fqn) => {
                            /*
                            let relay = Relay::new(fqn);
                            let (addr, _) = ctx.spawn_agent(relay, ());
                            let record = SessionRecord {
                                relay: addr,
                                channel,
                            };
                            self.incoming.relays.insert(session_key, record);
                            */
                        }
                        Request::Action(action) => {}
                        Request::Unsubscribe => {}
                    }
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

pub struct Session {
    sub: Unique<OpenSession>,
}

pub struct ConnectionSender {
    peer_id: PeerId,
    session_id: SessionId,
    recipient: Recipient<ForwardRequest>,
}

impl ConnectionSender {
    pub fn send(&self, request: Request) -> Result<()> {
        let request = ForwardRequest {
            peer_id: self.peer_id,
            session_id: self.session_id,
            request,
        };
        self.recipient.send(request)
    }
}

pub struct OpenSession {
    peer_id: PeerId,
    recipient: Recipient<Response>,
}

impl Subscription for OpenSession {
    type State = ConnectionSender;
}

#[async_trait]
impl ManageSubscription<OpenSession> for Connector {
    async fn subscribe(
        &mut self,
        sub: Unique<OpenSession>,
        ctx: &mut Context<Self>,
    ) -> Result<ConnectionSender> {
        let connection = Session { sub: sub.clone() };
        let session_id = self.outgoing.sessions.insert(connection);
        let connection = ConnectionSender {
            peer_id: sub.peer_id.clone(),
            session_id,
            recipient: ctx.recipient(),
        };
        self.outgoing.session_ids.insert(sub, session_id);
        Ok(connection)
    }

    async fn unsubscribe(
        &mut self,
        sub: Unique<OpenSession>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        let id = self.outgoing.session_ids.remove(&sub);
        if let Some(id) = id {
            self.outgoing.sessions.remove(id);
        }
        Ok(())
    }
}

struct ForwardRequest {
    peer_id: PeerId,
    session_id: SessionId,
    request: Request,
}

#[async_trait]
impl OnEvent<ForwardRequest> for Connector {
    async fn handle(&mut self, event: ForwardRequest, _ctx: &mut Context<Self>) -> Result<()> {
        let swarm = self.swarm.get_mut()?.behaviour_mut();
        let envelope = Envelope {
            session_id: event.session_id,
            value: event.request,
        };
        let _req_id = swarm
            .request_response
            .send_request(&event.peer_id, envelope);
        Ok(())
    }
}

struct ForwardResponse {
    session_key: SessionKey,
    response: Response,
}

#[async_trait]
impl OnEvent<ForwardResponse> for Connector {
    async fn handle(&mut self, event: ForwardResponse, _ctx: &mut Context<Self>) -> Result<()> {
        let swarm = self.swarm.get_mut()?.behaviour_mut();
        let session_id = event.session_key.session_id;
        let envelope = Envelope {
            session_id,
            value: event.response,
        };
        // TODO: Send an envelope
        /*
        swarm
            .request_response
            .send_response(&event.peer_id, envelope);
        */
        Ok(())
    }
}
