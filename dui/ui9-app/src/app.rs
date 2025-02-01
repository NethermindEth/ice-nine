use crate::protocol::UiEvent;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, RunAgent, Standalone};
use crb::core::{mpsc, Slot};
use crb::runtime::InteractiveRuntime;
use crb::superagent::{Drainer, OnItem, Supervisor, SupervisorSession};
use ui9_dui::subscriber::SubEvent;
use ui9_dui::tracers::peer::{Peer, PeerEvent};
use ui9_dui::utils::to_drainer;
use ui9_dui::Sub;

pub struct AppLink {
    pub address: Address<App>,
    pub events_rx: Slot<mpsc::UnboundedReceiver<UiEvent>>,
}

impl AppLink {
    pub fn try_recv(&mut self) -> Result<UiEvent> {
        let event = self.events_rx.get_mut()?.try_recv()?;
        Ok(event)
    }

    pub fn drainer(&mut self) -> Result<Drainer<UiEvent>> {
        let rx = self.events_rx.take()?;
        Ok(to_drainer(rx))
    }
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
            events_rx: Slot::filled(events_rx),
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
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for App {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let events = self.peers.events()?;
        ctx.assign(events, (), ());
        Ok(Next::events())
    }
}

#[async_trait]
impl OnItem<SubEvent<Peer>> for App {
    async fn on_item(
        &mut self,
        event: SubEvent<Peer>,
        _: (),
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        match event {
            SubEvent::State(peers) => {
                let event = UiEvent::SetState { peers };
                self.events_tx.send(event)?;
            }
            SubEvent::Event(_) => {
                let event = UiEvent::StateChanged;
                self.events_tx.send(event)?;
            }
            SubEvent::Lost => {}
        }
        Ok(())
    }
}
