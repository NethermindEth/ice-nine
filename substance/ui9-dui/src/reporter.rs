use crate::tracers::live::Live;
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::StreamSession;

pub struct Reporter {
    live: Pub<Live>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            live: Pub::unified(),
        }
    }
}

impl Agent for Reporter {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Reporter {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.live.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Live>> for Reporter {
    async fn handle(&mut self, msg: Act<Live>, ctx: &mut Context<Self>) -> Result<()> {
        self.live.event(msg.action.into());
        Ok(())
    }
}
