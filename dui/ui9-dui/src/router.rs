use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next};
use crb::superagent::{Drainer, OnItem, Supervisor, SupervisorSession};
use libp2p::{PeerId, Stream, StreamProtocol};
use libp2p_stream::Control;

static PROTOCOL: StreamProtocol = StreamProtocol::new("/ui9-flow");

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
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Router {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let streams = self.control.accept(PROTOCOL.clone())?;
        let drainer = Drainer::new(streams);
        ctx.assign(drainer, (), ());
        Ok(Next::events())
    }
}

#[async_trait]
impl OnItem<(PeerId, Stream)> for Router {
    async fn on_item(
        &mut self,
        event: (PeerId, Stream),
        _: (),
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        // TODO: Spawn a relay here
        Ok(())
    }
}
