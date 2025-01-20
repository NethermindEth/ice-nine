use crate::events::EventsDrainer;
use crate::state::AppFrame;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next, Standalone};
use crb::core::mpsc;
use crb::superagent::{Supervisor, SupervisorSession};

pub struct App {
    sender: mpsc::UnboundedSender<AppFrame>,
}

impl App {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<AppFrame>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let this = Self { sender: tx };
        (this, rx)
    }
}

impl Standalone for App {}

impl Supervisor for App {
    type GroupBy = ();
}

impl Agent for App {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
    }
}

struct Configure;

#[async_trait]
impl Duty<Configure> for App {
    async fn handle(&mut self, _: Configure, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let drainer = EventsDrainer::new(&*ctx);
        ctx.spawn_agent(drainer, ());

        // TODO: Launch the command

        Ok(Next::events())
    }
}
