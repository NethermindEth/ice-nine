use super::{Act, LocalPlayer, PlayerState};
use crate::flow::Flow;
use crate::relay::RemotePlayer;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent, StopRecipient};
use crb::runtime::{InteractiveRuntime, Runtime};
use crb::superagent::{EventBridge, StreamSession, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use libp2p::PeerId;
use std::sync::LazyLock;

pub trait PlayerGenerator {
    fn new_player<F: Flow>(
        &self,
        peer_id: Option<PeerId>,
        state: PlayerState<F>,
    ) -> (Box<dyn Runtime>, StopRecipient<Act<F>>);
}

struct LocalGenerator;

impl PlayerGenerator for LocalGenerator {
    fn new_player<F: Flow>(
        &self,
        peer_id: Option<PeerId>,
        state: PlayerState<F>,
    ) -> (Box<dyn Runtime>, StopRecipient<Act<F>>) {
        let runtime: Box<dyn Runtime>;
        let recipient;
        if let Some(peer_id) = peer_id {
            let player = RemotePlayer::new(peer_id, state);
            let agent = RunAgent::new(player);
            recipient = agent.address().to_stop_address().to_stop_recipient();
            runtime = Box::new(agent);
        } else {
            let player = LocalPlayer::new(state);
            let agent = RunAgent::new(player);
            recipient = agent.address().to_stop_address().to_stop_recipient();
            runtime = Box::new(agent);
        }
        (runtime, recipient)
    }
}

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubClientLink {
    hub: Address<HubClient>,
}

static SUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

impl HubClient {
    pub fn spawn_player<F: Flow>(
        peer_id: Option<PeerId>,
        state: PlayerState<F>,
    ) -> StopRecipient<Act<F>> {
        let (runtime, recipient) = LocalGenerator.new_player(peer_id, state);
        HubClient::add_player(runtime);
        recipient
    }

    pub fn add_player(runtime: Box<dyn Runtime>) {
        let delegate = Delegate { runtime };
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
