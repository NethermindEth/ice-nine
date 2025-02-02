use crate::protocol::UiEvent;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent, Standalone};
use crb::core::{mpsc, Slot, Msg};
use crb::runtime::InteractiveRuntime;
use crb::superagent::{Drainer, Supervisor, SupervisorSession};
use std::collections::BTreeMap;
use ui9_dui::subscriber::{drainer, SubEvent};
use ui9_dui::tracers::peer::{Peer, PeerEvent, PeerId};
use ui9_dui::tracers::tree::Tree;
use ui9_dui::{Sub, Flow};

pub struct AppLink<E: Msg> {
    pub address: Address<App<E>>,
    pub events_rx: Slot<mpsc::UnboundedReceiver<E>>,
}

impl<E: Msg> AppLink<E> {
    pub fn try_recv(&mut self) -> Result<E> {
        let event = self.events_rx.get_mut()?.try_recv()?;
        Ok(event)
    }

    pub fn drainer(&mut self) -> Result<Drainer<E>> {
        let rx = self.events_rx.take()?;
        Ok(drainer::from_mpsc(rx))
    }
}

pub struct App<E> {
    peers: Sub<Peer>,
    tree: Sub<Tree>,
    ui_events_tx: mpsc::UnboundedSender<E>,
}

impl<E: Msg> App<E> {
    pub fn new() -> (RunAgent<Self>, AppLink<E>) {
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let agent = Self {
            peers: Sub::unified(None),
            tree: Sub::unified(None),
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

impl<E: Msg> Standalone for App<E> {}

impl<E: Msg> Supervisor for App<E> {
    type GroupBy = ();
}

impl<E: Msg> Agent for App<E> {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<E: Msg> DoAsync<Initialize> for App<E> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        /*
        let events = self.peers.events()?;
        ctx.assign(events, (), ());

        let events = self.tree.events()?;
        ctx.assign(events, (), ());
        */
        Ok(Next::events())
    }
}

#[async_trait]
impl<E, F> OnEvent<SubEvent<F>> for App<E>
where
    E: Msg + From<SubEvent<F>>,
    F: Flow,
{
    async fn handle(&mut self, event: SubEvent<F>, ctx: &mut Context<Self>) -> Result<()> {
        self.ui_events_tx.send(event.into())
            .map_err(|_| anyhow!("Can't forward an event to UI."))?;
        Ok(())
    }
}
