use crate::connector::Connector;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, OnEvent, Standalone};
use crb::runtime::Runtime;
use crb::superagent::{Supervisor, SupervisorSession};
use std::sync::OnceLock;

// TODO: Use `Link` instead?
pub static HUB: OnceLock<Address<Hub>> = OnceLock::new();

pub struct Hub {}

impl Hub {
    pub fn activate() {
        let hub = Hub {};
        let address = hub.spawn();
        if let Err(address) = HUB.set(address) {
            // Interrupt since hub is spawned already.
            address.interrupt();
        }
    }

    pub async fn deactivate() {
        if let Some(mut address) = HUB.get().cloned() {
            address.interrupt();
            address.join().await;
        }
    }
}

impl Standalone for Hub {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Connector,
    Relay,
}

impl Supervisor for Hub {
    type GroupBy = Group;
}

impl Agent for Hub {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Hub {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let connector = Connector::new();
        ctx.spawn_agent(connector, Group::Connector);
        Ok(Next::events())
    }
}

pub struct Delegate {
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for Hub {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, Group::Relay);
        Ok(())
    }
}
