use crate::protocol::UiEvent;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, RunAgent, Standalone};
use crb::core::mpsc;
use crb::runtime::InteractiveRuntime;
use crb::superagent::{Supervisor, SupervisorSession};
use ui9_dui::tracers::peer::Peer;
use ui9_dui::Sub;

pub struct AppLink {
    pub address: Address<App>,
    pub events_rx: mpsc::UnboundedReceiver<UiEvent>,
}

pub struct App {
    peers: Sub<Peer>,
    events_tx: mpsc::UnboundedSender<UiEvent>,
}

impl App {
    pub fn new() -> (RunAgent<Self>, AppLink) {
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let agent = Self {
            peers: Sub::unified(),
            events_tx,
        };
        let runtime = RunAgent::new(agent);
        let link = AppLink {
            address: runtime.address().clone(),
            events_rx,
        };
        (runtime, link)
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
        let peers = self.peers.state().clone();
        let event = UiEvent::SetState { peers };
        self.events_tx.send(event)?;
        Ok(Next::events())
    }
}
