use crate::protocol::UiEvent;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent, Standalone};
use crb::core::{mpsc, Slot};
use crb::runtime::InteractiveRuntime;
use crb::superagent::{Drainer, Supervisor, SupervisorSession};
use ui9_dui::subscriber::SubEvent;
use ui9_dui::tracers::peer::{Peer, PeerEvent, PeerId};
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
    ui_events_tx: mpsc::UnboundedSender<UiEvent>,
}

impl App {
    pub fn new() -> (RunAgent<Self>, AppLink) {
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let agent = Self {
            peers: Sub::unified(),
            ui_events_tx: events_tx,
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

impl App {
    fn subscribe_to_peer(&mut self, peer: PeerId) {
        log::info!("Subscribing to peer's tree: {peer}");
    }
}

#[async_trait]
impl OnEvent<SubEvent<Peer>> for App {
    async fn handle(&mut self, event: SubEvent<Peer>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                for (peer_id, _) in state.borrow().peers.iter() {
                    self.subscribe_to_peer(*peer_id);
                }
                let ui_event = UiEvent::SetState { peers: state };
                self.ui_events_tx.send(ui_event)?;
            }
            SubEvent::Event(event) => {
                match &event {
                    PeerEvent::AddPeer { peer_id, .. } => {
                        self.subscribe_to_peer(*peer_id);
                    }
                    _ => {}
                }
                let ui_event = UiEvent::StateChanged;
                self.ui_events_tx.send(ui_event)?;
            }
            SubEvent::Lost => {}
        }
        Ok(())
    }
}
