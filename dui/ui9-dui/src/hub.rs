use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, OnEvent, Standalone};
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

impl Supervisor for Hub {
    type GroupBy = ();
}

impl Agent for Hub {
    type Context = SupervisorSession<Self>;
}

pub struct Delegate {
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for Hub {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, ());
        Ok(())
    }
}
