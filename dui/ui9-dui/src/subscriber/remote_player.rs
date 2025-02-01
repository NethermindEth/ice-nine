use super::{Act, PlayerState};
use crate::connector::OpenSession;
use crate::hub::Hub;
use crate::protocol;
use crate::Flow;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::StateEntry;
use libp2p::PeerId;

pub struct RemotePlayer<F: Flow> {
    peer_id: PeerId,
    state: PlayerState<F>,
    session: Slot<StateEntry<OpenSession>>,
}

impl<F: Flow> RemotePlayer<F> {
    pub fn new(peer_id: PeerId, state: PlayerState<F>) -> Self {
        Self {
            peer_id,
            state,
            session: Slot::empty(),
        }
    }
}

impl<F: Flow> Agent for RemotePlayer<F> {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<F: Flow> DoAsync<Initialize> for RemotePlayer<F> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let hub = Hub::link()?;
        let session = hub.connector.open_session(self.peer_id, &ctx).await?;
        self.session.fill(session)?;

        Ok(Next::do_async(Subscribe))
    }

    // TODO: Fallback to reconnect
}

struct Subscribe;

#[async_trait]
impl<F: Flow> DoAsync<Subscribe> for RemotePlayer<F> {
    async fn handle(&mut self, _: Subscribe, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let conn = self.session.get_mut()?;
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
