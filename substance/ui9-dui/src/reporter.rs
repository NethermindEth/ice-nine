use crate::tracers::event::{Event, EventData};
use crate::tracers::failure::{Failure, FailureData};
use crate::tracers::job::{Job, JobData, OperationId};
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::time::Instant;
use crb::superagent::{AgentBridge, StreamSession};
use std::sync::LazyLock;

static LOG_BRIDGE: LazyLock<AgentBridge<Reporter>> = LazyLock::new(|| AgentBridge::new());

pub struct Operation {
    timestamp: DateTime<Local>,
    started: Instant,
    id: OperationId,
    /// If taken (empty) the task is considered as completed
    task: Option<String>,
}

impl Drop for Operation {
    fn drop(&mut self) {
        if let Some(message) = self.task.take() {
            // Operations must be explicitly completed
            let data = FailureData { message };
            self.act_failure(data);
        }
    }
}

impl Operation {
    pub fn start(task: &str) -> Self {
        let id = OperationId::new();
        let mut this = Self {
            timestamp: Local::now(),
            started: Instant::now(),
            id,
            task: Some(task.into()),
        };
        this.act_job(JobData::Begin {
            id,
            task: task.into(),
        });
        this
    }

    pub fn failure(&mut self, reason: &str) {
        self.act_job(JobData::Failure {
            id: self.id,
            reason: reason.into(),
        });
    }

    pub fn end(mut self, message: &str) {
        self.task.take();
        self.send_end(message.into());
    }

    fn send_end(&mut self, message: String) {
        let duration = self.started.elapsed();
        self.act_job(JobData::End { id: self.id });
        self.act_event(EventData { message, duration });
    }

    fn act_job(&mut self, action: JobData) {
        let event = Act::<Job> { action };
        LOG_BRIDGE.event(event);
    }

    fn act_event(&mut self, action: EventData) {
        let event = Act::<Event> { action };
        LOG_BRIDGE.event(event);
    }

    fn act_failure(&mut self, action: FailureData) {
        let event = Act::<Failure> { action };
        LOG_BRIDGE.event(event);
    }
}

pub struct Reporter {
    job: Pub<Job>,
    event: Pub<Event>,
    failure: Pub<Failure>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            job: Pub::unified(),
            event: Pub::unified(),
            failure: Pub::unified(),
        }
    }

    /*
    pub fn log(msg: &str) {
        let event = Act {
            action: JobData::Message(msg.into()),
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
        ctx.consume_events(LOG_BRIDGE.events().await?);
        ctx.consume(self.job.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Job>> for Reporter {
    async fn handle(&mut self, msg: Act<Job>, _ctx: &mut Context<Self>) -> Result<()> {
        self.job.event(msg.action);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<Act<Event>> for Reporter {
    async fn handle(&mut self, msg: Act<Event>, _ctx: &mut Context<Self>) -> Result<()> {
        self.event.event(msg.action);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<Act<Failure>> for Reporter {
    async fn handle(&mut self, msg: Act<Failure>, _ctx: &mut Context<Self>) -> Result<()> {
        self.failure.event(msg.action);
        Ok(())
    }
}
