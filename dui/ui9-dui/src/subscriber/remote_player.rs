use super::{Act, PlayerState};
use crate::connector::OpenConnection;
use crate::hub::Hub;
use crate::protocol;
use crate::Flow;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::StateEntry;
use libp2p::PeerId;

pub struct RemotePlayer<F: Flow> {
    peer_id: PeerId,
    state: PlayerState<F>,
    connection: Slot<StateEntry<OpenConnection>>,
}

impl<F: Flow> RemotePlayer<F> {
    pub fn new(peer_id: PeerId, state: PlayerState<F>) -> Self {
        Self {
            peer_id,
            state,
            connection: Slot::empty(),
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
        self.connection.fill(connection)?;

        Ok(Next::duty(Subscribe))
    }

    // TODO: Fallback to reconnect
}

struct Subscribe;

#[async_trait]
impl<F: Flow> Duty<Subscribe> for RemotePlayer<F> {
    async fn handle(&mut self, _: Subscribe, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let conn = self.connection.get_mut()?;
        let fqn = self.state.fqn.clone();
        let req = protocol::Request::Subscribe(fqn);
        conn.state.send(req)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl<F: Flow> OnEvent<protocol::Response> for RemotePlayer<F> {
    async fn handle(&mut self, _res: protocol::Response, _ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl<F: Flow> OnEvent<Act<F>> for RemotePlayer<F> {
    async fn handle(&mut self, action: Act<F>, _ctx: &mut Context<Self>) -> Result<()> {
        Err(anyhow!(
            "Not yet implemented: sending action to a remote component"
        ))
    }
}
