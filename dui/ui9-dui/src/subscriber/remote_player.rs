use super::Ported;
use crate::Flow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next};
use crb::core::watch;
use libp2p::PeerId;
use ui9::names::Fqn;

pub struct RemotePlayer<F: Flow> {
    peer_id: PeerId,
    fqn: Fqn,
    state: watch::Sender<Ported<F>>,
}

impl<F: Flow> RemotePlayer<F> {
    pub fn new(peer_id: PeerId, fqn: Fqn, state: watch::Sender<Ported<F>>) -> Self {
        Self {
            peer_id,
            fqn,
            state,
        }
    }
}

impl<F: Flow> Agent for RemotePlayer<F> {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<F: Flow> Duty<Initialize> for RemotePlayer<F> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
