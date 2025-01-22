use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Duty, Next};
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

pub struct Connector {}

impl Connector {
    pub fn new() -> Self {
        Self {}
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

impl Agent for Connector {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
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

        let event_loop = EventLoop { swarm };
        Ok(Next::do_async(event_loop))
    }
}

#[derive(Deref, DerefMut)]
struct EventLoop {
    swarm: Swarm<Ui9Behaviour>,
}

#[async_trait]
impl DoAsync<EventLoop> for Connector {
    async fn repeat(&mut self, swarm: &mut EventLoop) -> Result<Option<Next<Self>>> {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(event) => {
                    match event {
                        Ui9BehaviourEvent::Mdns(mdns::Event::Discovered(list)) => {
                            for (peer_id, _multiaddr) in list {
                                println!("UI9 node connected: {peer_id}");
                                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                            }
                        },
                        Ui9BehaviourEvent::Mdns(mdns::Event::Expired(list)) => {
                            for (peer_id, _multiaddr) in list {
                                println!("UI9 node disconnected: {peer_id}");
                                swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                            }
                        },
                        Ui9BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        }) => {
                            println!(
                                "Got message: '{}' with id: {id} from peer: {peer_id}",
                                String::from_utf8_lossy(&message.data),
                            );
                        },
                        Ui9BehaviourEvent::RequestResponse(_) => {
                        },
                        other => {
                            println!("Not handeled p2p behaviour event: {other:?}");
                        }
                    }
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                }
                other => {
                    println!("Not handeled p2p event: {other:?}");
                }
            }
        }
        Ok(None)
    }
}
