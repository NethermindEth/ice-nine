use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Duty, ManagedContext, Next, OnEvent};
use crb::core::Slot;
use derive_more::{Deref, DerefMut};
use futures::stream::StreamExt;
use libp2p::{
    gossipsub, mdns, noise,
    request_response::{self, OutboundRequestId, ProtocolSupport, ResponseChannel},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, StreamProtocol, Swarm,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::select;

type ReqRespEvent = request_response::Event<Ui9Request, Ui9Response>;

pub struct Connector {
    swarm: Slot<Swarm<Ui9Behaviour>>,
}

impl Connector {
    pub fn new() -> Self {
        Self {
            swarm: Slot::empty(),
        }
    }
}

#[derive(NetworkBehaviour)]
struct Ui9Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    request_response: request_response::cbor::Behaviour<Ui9Request, Ui9Response>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ui9Request(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ui9Response(Vec<u8>);

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

        self.swarm.fill(swarm);
        Ok(Next::events())
    }
}

impl Connector {
    async fn route_swarm_event(
        &mut self,
        event: SwarmEvent<Ui9BehaviourEvent>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        let swarm = self.swarm.get_mut()?;
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
                _ => {}
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local node is listening on {address}");
            }
            other => {
                println!("Not handeled p2p event: {other:?}");
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<mdns::Event> for Connector {
    async fn handle(&mut self, event: mdns::Event, ctx: &mut Context<Self>) -> Result<()> {
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
    async fn handle(&mut self, event: gossipsub::Event, ctx: &mut Context<Self>) -> Result<()> {
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
impl OnEvent<ReqRespEvent> for Connector {
    async fn handle(&mut self, event: ReqRespEvent, ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}
