use crate::events::EventsDrainer;
use crate::state::AppState;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Agent, Context, DoAsync, DoSync, InContext, Next, OnEvent, Standalone, Supervisor,
    SupervisorSession,
};
use crb::core::Slot;
/*
use crossterm::event::Event;
use ratatui::DefaultTerminal;
*/

pub struct App {
    // terminal: Slot<DefaultTerminal>,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            // terminal: Slot::empty(),
            state: AppState::new(),
        }
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
        /*
        let terminal = ratatui::try_init()?;
        self.terminal.fill(terminal)?;
        */
        let address = ctx.address().clone();
        let drainer = EventsDrainer::new(address);
        ctx.spawn_agent(drainer, ());
        Ok(Next::do_sync(Render))
    }
}

/*
#[async_trait]
impl OnEvent<Event> for App {
    type Error = Error;

    async fn handle(&mut self, event: Event, ctx: &mut Self::Context) -> Result<()> {
        let next_state = match event {
            Event::Key(_event) => Next::do_async(Terminate),
            _ => Next::do_sync(Render),
        };
        ctx.do_next(next_state);
        Ok(())
    }
}
*/

struct Render;

impl DoSync<Render> for App {
    fn once(&mut self, _: &mut Render) -> Result<Next<Self>> {
        /*
        let terminal = self.terminal.get_mut()?;
        terminal.draw(|frame| self.state.render(frame))?;
        */
        Ok(Next::events())
    }
}

struct Terminate;

#[async_trait]
impl DoAsync<Terminate> for App {
    async fn once(&mut self, _: &mut Terminate) -> Result<Next<Self>> {
        // ratatui::try_restore()?;
        Ok(Next::done())
    }
}
