use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next, Standalone};
use crb::superagent::{Supervisor, SupervisorSession};
use ui9_dui::tracers::peer::Peer;
use ui9_dui::Sub;

pub struct App {
    peers: Sub<Peer>,
}

impl App {
    pub fn new() -> Self {
        Self {
            peers: Sub::unified(),
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
    async fn handle(&mut self, _: Initialize, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
