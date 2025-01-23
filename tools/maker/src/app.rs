use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Standalone, Agent, Next, Context, Duty};
use crb::superagent::{Supervisor, SupervisorSession};

pub struct App {
}

impl App {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Standalone for App {}

impl Supervisor for App {
    type GroupBy = ();
}

impl Agent for App {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for App {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
