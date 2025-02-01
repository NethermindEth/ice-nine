use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent};
use crb::runtime::Runtime;
use crb::superagent::{EventBridge, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use std::sync::LazyLock;

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubClientLink {
    hub: Address<HubClient>,
}

static SUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

impl HubClient {
    pub fn add_player(runtime: impl Runtime) {
        let delegate = Delegate {
            runtime: Box::new(runtime),
        };
        SUB_BRIDGE.send(delegate);
    }
}

pub struct HubClient {}

impl HubClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl Supervisor for HubClient {
    type GroupBy = ();
}

impl Agent for HubClient {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for HubClient {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        log::debug!("HubClient starting...");
        SUB_BRIDGE.subscribe(&ctx);
        log::debug!("HubClient active");
        Ok(Next::events())
    }
}

pub struct Delegate {
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubClient {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, ());
        Ok(())
    }
}
