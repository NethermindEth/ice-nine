use crate::args::RunArgs;
use crate::command::{CommandWatcher, UiioEvent};
use crate::events::EventsDrainer;
use crate::state::{AppFrame, AppState};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, ManagedContext, Next, OnEvent, Standalone};
use crb::core::mpsc;
use crb::core::time::Duration;
use crb::superagent::{Relation, Supervisor, SupervisorSession, Timer};

pub struct App {
    args: RunArgs,
    state: AppState,
    sender: mpsc::UnboundedSender<AppFrame>,
    interval: Timer<Tick>,
}

impl App {
    pub fn new(args: RunArgs) -> (Self, mpsc::UnboundedReceiver<AppFrame>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut interval = Timer::new(Tick);
        interval.set_repeat(true);
        let this = Self {
            args,
            state: AppState::new(),
            sender: tx,
            interval,
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

    fn finished(&mut self, rel: &Relation<Self>, ctx: &mut Context<Self>) {
        if rel.group == Group::Watcher {
            // ctx.shutdown();
        }
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
        let watcher = CommandWatcher::new(self.args.clone(), &ctx);
        ctx.spawn_agent(watcher, Group::Watcher);

        let drainer = EventsDrainer::new(&ctx);
        ctx.spawn_agent(drainer, Group::Drainer);

        self.interval.add_listener(&ctx);
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

#[async_trait]
impl OnEvent<UiioEvent> for App {
    async fn handle(&mut self, event: UiioEvent, ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}
