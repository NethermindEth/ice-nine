use crate::tracers::live::{Live, LiveData, OperationId};
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{EventBridge, StreamSession};
use std::sync::LazyLock;

static LOG_BRIDGE: LazyLock<EventBridge<Act<Live>>> = LazyLock::new(|| EventBridge::new());

pub struct Operation {
    id: OperationId,
    /// If taken (empty) the task is considered as completed
    task: Option<String>,
}

impl Drop for Operation {
    fn drop(&mut self) {
        if let Some(message) = self.task.take() {
            self.send_end(message);
        }
    }
}

impl Operation {
    pub fn start(task: &str) -> Self {
        let id = OperationId::new();
        let mut this = Self {
            id,
            task: Some(task.into()),
        };
        this.act(LiveData::Begin {
            id,
            task: task.into(),
        });
        this
    }

    pub fn failure(&mut self, reason: &str) {
        self.act(LiveData::Failure {
            id: self.id,
            reason: reason.into(),
        });
    }

    pub fn end(mut self, message: &str) {
        self.task.take();
        self.send_end(message.into());
    }

    fn send_end(&mut self, message: String) {
        self.act(LiveData::End {
            id: self.id,
            message,
        });
    }

    fn act(&mut self, action: LiveData) {
        let event = Act { action };
        LOG_BRIDGE.send(event);
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

    /*
    pub fn log(msg: &str) {
        let event = Act {
            action: LiveData::Message(msg.into()),
        };
        LOG_BRIDGE.send(event);
    }
    */
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
        self.live.event(msg.action);
        Ok(())
    }
}
