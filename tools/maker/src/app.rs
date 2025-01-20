use crate::events::EventsDrainer;
use crate::state::{AppFrame, AppState};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next, OnEvent, Standalone};
use crb::core::mpsc;
use crb::core::time::Duration;
use crb::superagent::{IntervalSwitch, Relation, Supervisor, SupervisorSession};

pub struct App {
    state: AppState,
    sender: mpsc::UnboundedSender<AppFrame>,
    interval: IntervalSwitch<Tick>,
}

impl App {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<AppFrame>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let this = Self {
            state: AppState::new(),
            sender: tx,
            interval: IntervalSwitch::new(Duration::from_millis(1_000), Tick),
        };
        (this, rx)
    }
}

impl Standalone for App {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Watcher,
    Drainer,
}

impl Supervisor for App {
    type GroupBy = Group;

    fn finished(&mut self, rel: &Relation<Self>, _ctx: &mut Context<Self>) {
        // TODO: Terminate if the Watcher is terminated
    }
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
        let drainer = EventsDrainer::new(&*ctx);
        ctx.spawn_agent(drainer, Group::Drainer);

        // TODO: Launch the command

        self.interval.add_listener(&*ctx);
        self.interval.on();
        Ok(Next::events())
    }
}

#[derive(Clone)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for App {
    async fn handle(&mut self, _: Tick, ctx: &mut Context<Self>) -> Result<()> {
        self.state.count_up();
        let frame = self.state.frame();
        self.sender.send(frame)?;
        Ok(())
    }
}
