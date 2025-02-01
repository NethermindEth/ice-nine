use super::PROTOCOL;
use super::relay_player::RelayPlayer;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{Drainer, Supervisor, SupervisorSession};
use libp2p::{PeerId, Stream};
use libp2p_stream::Control;

pub struct Router {
    control: Control,
}

impl Router {
    pub fn new(control: Control) -> Self {
        Self { control }
    }
}

impl Supervisor for Router {
    type GroupBy = ();
}

impl Agent for Router {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Router {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let streams = self.control.accept(PROTOCOL.clone())?;
        let drainer = Drainer::new(streams);
        ctx.assign(drainer, (), ());
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<(PeerId, Stream)> for Router {
    async fn handle(&mut self, event: (PeerId, Stream), ctx: &mut Context<Self>) -> Result<()> {
        let relay = RelayPlayer::new(event.1);
        ctx.spawn_agent(relay, ());
        Ok(())
    }
}
