use super::{Act, LocalGenerator, PlayerGenerator, PlayerState};
use crate::flow::Flow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent, StopRecipient};
use crb::runtime::Runtime;
use crb::superagent::{EventBridge, StreamSession, Supervisor, SupervisorSession};
use libp2p::PeerId;
use std::sync::LazyLock;

static SUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

impl HubClient {
    pub fn add_player(runtime: impl Runtime) {
        let delegate = Delegate {
            runtime: Box::new(runtime),
        };
        SUB_BRIDGE.send(delegate);
    }
}

pub struct HubClient<G = LocalGenerator> {
    generator: G,
}

impl<G> HubClient<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl Supervisor for HubClient {
    type BasedOn = StreamSession<Self>;
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
        ctx.consume(SUB_BRIDGE.events().await?);
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
