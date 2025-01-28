use crate::connector::Connector;
use crate::tracers::peer::{Peer, PeerEvent};
use crate::Sub;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, OnEvent};
use crb::superagent::{Interval, OnItem, Supervisor, SupervisorSession};

/// A hub server that keep information about remote components.
pub struct Relay {
    connector: Address<Connector>,
    peer_listener: Sub<Peer>,
    interval: Interval<Tick>,
}

impl Relay {
    pub fn new(connector: Address<Connector>) -> Self {
        Self {
            connector,
            peer_listener: Sub::unified(),
            interval: Interval::default(),
        }
    }
}

impl Supervisor for Relay {
    type GroupBy = ();
}

impl Agent for Relay {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Relay {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.interval.enable(&ctx);
        let events = self.peer_listener.events()?;
        ctx.assign(events, (), ());
        Ok(Next::events())
    }
}

#[derive(Clone, Default)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for Relay {
    async fn handle(&mut self, _: Tick, _ctx: &mut Context<Self>) -> Result<()> {
        let state = self.peer_listener.state();
        println!("PEERS: {:?}", *state);
        Ok(())
    }
}

#[async_trait]
impl OnItem<PeerEvent> for Relay {
    async fn on_item(&mut self, event: PeerEvent, _: (), _ctx: &mut Context<Self>) -> Result<()> {
        println!("PEERS UPDATE: {:?}", event);
        Ok(())
    }
}
