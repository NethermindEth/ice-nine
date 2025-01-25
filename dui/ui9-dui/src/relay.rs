use crate::connector::Connector;
use crate::tracers::PeerListener;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Duty, Next, OnEvent};
use crb::superagent::Timer;

/// A hub server that keep information about remote components.
pub struct Relay {
    connector: Address<Connector>,
    peer_listener: PeerListener,
    interval: Timer<Tick>,
}

impl Relay {
    pub fn new(connector: Address<Connector>) -> Self {
        Self {
            connector,
            peer_listener: PeerListener::new(None),
            interval: Timer::new(Tick),
        }
    }
}

impl Agent for Relay {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Relay {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.interval.add_listener(&ctx);
        self.interval.set_repeat(true);
        self.interval.on();
        Ok(Next::events())
    }
}

#[derive(Clone)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for Relay {
    async fn handle(&mut self, _: Tick, _ctx: &mut Context<Self>) -> Result<()> {
        let state = self.peer_listener.state();
        println!("PEERS: {:?}", *state);
        Ok(())
    }
}
