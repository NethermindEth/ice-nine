use crate::protocol::UiEvent;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent, Standalone};
use crb::core::{mpsc, Msg, Slot};
use crb::runtime::InteractiveRuntime;
use crb::superagent::{Drainer, Supervisor, SupervisorSession};
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::DerefMut;
use ui9::names::Fqn;
use ui9_dui::subscriber::{drainer, SubEvent};
use ui9_dui::tracers::peer::{Peer, PeerEvent, PeerId};
use ui9_dui::tracers::tree::Tree;
use ui9_dui::{Flow, Listener, Sub, Subscriber};

pub trait AnySub: Send {}

impl<F: Subscriber> AnySub for Sub<F>
where
    F: Subscriber,
    F::Driver: Send,
{
}

pub struct AppLink<E: Msg> {
    pub address: Address<App<E>>,
    pub events_rx: Slot<mpsc::UnboundedReceiver<E>>,
}

impl<E: Msg> AppLink<E> {
    /// A method for immediate-state loops
    pub fn try_recv(&mut self) -> Result<E> {
        let event = self.events_rx.get_mut()?.try_recv()?;
        Ok(event)
    }

    /// A methods for actor-based reactors
    pub fn drainer(&mut self) -> Result<Drainer<E>> {
        let rx = self.events_rx.take()?;
        Ok(drainer::from_mpsc(rx))
    }
}

pub struct App<E> {
    subs: HashMap<usize, Box<dyn AnySub>>,
    ui_events_tx: mpsc::UnboundedSender<E>,
}

impl<E: Msg> App<E> {
    pub fn new() -> (RunAgent<Self>, AppLink<E>) {
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let agent = Self {
            subs: HashMap::new(),
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
    type GroupBy = usize;
}

impl<E: Msg> Agent for App<E> {
    type Context = SupervisorSession<Self>;
}

struct Subscribe<F> {
    idx: usize,
    flow: PhantomData<F>,
    fqn: Fqn,
}

#[async_trait]
impl<E, F> OnEvent<Subscribe<F>> for App<E>
where
    E: Msg + From<SubEvent<F>>,
    F: Subscriber,
    F::Driver: DerefMut<Target = Listener<F>> + Send,
{
    async fn handle(&mut self, msg: Subscribe<F>, ctx: &mut Context<Self>) -> Result<()> {
        let mut sub = Sub::<F>::new(None, msg.fqn);
        let events = sub.events()?;
        ctx.assign(events, msg.idx, ());
        self.subs.insert(msg.idx, Box::new(sub));
        Ok(())
    }
}

#[async_trait]
impl<E, F> OnEvent<SubEvent<F>> for App<E>
where
    E: Msg + From<SubEvent<F>>,
    F: Flow,
{
    async fn handle(&mut self, event: SubEvent<F>, ctx: &mut Context<Self>) -> Result<()> {
        self.ui_events_tx
            .send(event.into())
            .map_err(|_| anyhow!("Can't forward an event to UI."))?;
        Ok(())
    }
}
