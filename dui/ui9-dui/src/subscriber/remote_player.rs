use super::TelePorted;
use crate::connector::OpenConnection;
use crate::hub::Hub;
use crate::protocol;
use crate::Flow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next, OnEvent};
use crb::core::{watch, Slot};
use crb::superagent::StateEntry;
use libp2p::PeerId;
use ui9::names::Fqn;

pub struct RemotePlayer<F: Flow> {
    peer_id: PeerId,
    fqn: Fqn,
    state: watch::Sender<TelePorted<F>>,
    entry: Slot<StateEntry<OpenConnection>>,
}

impl<F: Flow> RemotePlayer<F> {
    pub fn new(peer_id: PeerId, fqn: Fqn, state: watch::Sender<TelePorted<F>>) -> Self {
        Self {
            peer_id,
            fqn,
            state,
            entry: Slot::empty(),
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
        let hub = Hub::link()?;
        let connection = hub.connector.open_connection(self.peer_id, &ctx).await?;

        Ok(Next::events())
    }

    // TODO: Fallback to reconnect
}

#[async_trait]
impl<F: Flow> OnEvent<protocol::Response> for RemotePlayer<F> {
    async fn handle(&mut self, _res: protocol::Response, _ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}
