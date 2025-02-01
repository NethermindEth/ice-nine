use super::{Act, PlayerState};
use crate::drainer::{to_drainer, MessageSink};
use crate::hub::Hub;
use crate::protocol::{Ui9Message, Ui9Request, Ui9Response};
use crate::router::PROTOCOL;
use crate::Flow;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{StateEntry, Supervisor, SupervisorSession};
use futures::SinkExt;
use libp2p::PeerId;

pub struct RemotePlayer<F: Flow> {
    peer_id: PeerId,
    state: PlayerState<F>,
    writer: Slot<MessageSink>,
}

impl<F: Flow> RemotePlayer<F> {
    pub fn new(peer_id: PeerId, state: PlayerState<F>) -> Self {
        Self {
            peer_id,
            state,
            writer: Slot::empty(),
        }
    }
}

impl<F: Flow> Supervisor for RemotePlayer<F> {
    type GroupBy = ();
}

impl<F: Flow> Agent for RemotePlayer<F> {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<F: Flow> DoAsync<Initialize> for RemotePlayer<F> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let hub = Hub::link()?;
        let mut control = hub.connector.get_control().await?;
        let stream = control.open_stream(self.peer_id, PROTOCOL.clone()).await?;
        let (drainer, writer) = to_drainer(stream);
        ctx.assign(drainer, (), ());

        let fqn = self.state.fqn.clone();
        self.send(fqn.into()).await?;

        self.writer.fill(writer)?;

        Ok(Next::events())
    }

    // TODO: Fallback to reconnect
}

impl<F: Flow> RemotePlayer<F> {
    async fn send(&mut self, request: Ui9Request) -> Result<()> {
        let writer = self.writer.get_mut()?;
        let message = Ui9Message::from(request);
        writer.send(message).await?;
        Ok(())
    }
}

#[async_trait]
impl<F: Flow> OnEvent<Result<Ui9Message>> for RemotePlayer<F> {
    async fn handle(&mut self, msg: Result<Ui9Message>, _ctx: &mut Context<Self>) -> Result<()> {
        match msg? {
            Ui9Message::Response(response) => {
                match response {
                    Ui9Response::State(state) => {
                        let unpacked_state = F::unpack_state(&state)?;
                        self.state.allocate_state(unpacked_state);
                    }
                    Ui9Response::Event(event) => {
                        let event = F::unpack_event(&event)?;
                        self.state.apply_event(event);
                    }
                }
                Ok(())
            }
            Ui9Message::Request(_) => Err(anyhow!(
                "Request is not expected for the remote player stream"
            )),
        }
    }
}

#[async_trait]
impl<F: Flow> OnEvent<Act<F>> for RemotePlayer<F> {
    async fn handle(&mut self, action: Act<F>, _ctx: &mut Context<Self>) -> Result<()> {
        let packed_action = F::pack_action(&action.action)?;
        self.send(packed_action.into());
        Ok(())
    }
}
