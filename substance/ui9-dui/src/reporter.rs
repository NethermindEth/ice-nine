use crate::tracers::live::{Live, LiveAction};
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::StreamSession;
use derive_more::{Deref, DerefMut, From};

#[derive(Deref, DerefMut, From)]
pub struct ReporterLink {
    address: Address<Reporter>,
}

impl ReporterLink {
    pub fn log(&self, msg: &str) {
        let event = Act {
            action: LiveAction::from(msg),
        };
        self.address.event(event).ok();
    }
}

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
