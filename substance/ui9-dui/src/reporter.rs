use crate::tracers::live::{Live, LiveAction};
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{EventBridge, StreamSession};
use derive_more::From;
use std::sync::LazyLock;

static LOG_BRIDGE: LazyLock<EventBridge<Act<Live>>> = LazyLock::new(|| EventBridge::new());

pub struct Reporter {
    live: Pub<Live>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            live: Pub::unified(),
        }
    }

    pub fn log(msg: &str) {
        let event = Act {
            action: LiveAction::from(msg),
        };
        LOG_BRIDGE.send(event);
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
        ctx.consume(LOG_BRIDGE.events().await?);
        ctx.consume(self.live.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Live>> for Reporter {
    async fn handle(&mut self, msg: Act<Live>, _ctx: &mut Context<Self>) -> Result<()> {
        self.live.event(msg.action.into());
        Ok(())
    }
}
