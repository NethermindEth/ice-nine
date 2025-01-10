use crate::events::EventsDrainer;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, InContext, Next, Standalone, Supervisor, SupervisorSession};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl Standalone for App {}

impl Supervisor for App {
    type GroupBy = ();
}

impl Agent for App {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::in_context(Configure)
    }
}

struct Configure;

#[async_trait]
impl InContext<Configure> for App {
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let address = ctx.address().clone();
        let drainer = EventsDrainer::new(address);
        ctx.spawn_agent(drainer, ());
        Ok(Next::events())
    }
}
