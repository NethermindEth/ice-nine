use crate::events::EventsDrainer;
use crate::state::AppState;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, DoSync, Next, OnEvent, RunAgent};
use crb::core::Slot;
use crb::runtime::InterruptionLevel;
use crb::superagent::{Supervisor, SupervisorSession};
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;
use ui9_maker::{AppLink, UiEvent};

pub struct AppTui {
    terminal: Slot<DefaultTerminal>,
    link: AppLink,
    state: AppState,
}

impl AppTui {
    pub fn new(link: AppLink) -> Self {
        Self {
            terminal: Slot::empty(),
            link,
            state: AppState::new(),
        }
    }
}

impl Supervisor for AppTui {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for AppTui {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for AppTui {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let terminal = ratatui::try_init()?;
        self.terminal.fill(terminal)?;

        // TODO: Use a drainer from CRB
        let drainer = EventsDrainer::new(&ctx);
        let mut runtime = RunAgent::new(drainer);
        runtime.level = InterruptionLevel::ABORT;
        ctx.spawn_runtime(runtime, ());

        let ui_events = self.link.drainer()?;
        ctx.assign(ui_events, (), ());

        Ok(Next::do_sync(Render))
    }
}

#[async_trait]
impl OnEvent<Event> for AppTui {
    async fn handle(&mut self, event: Event, ctx: &mut Context<Self>) -> Result<()> {
        let next_state = match event {
            Event::Key(event) => match event.code {
                KeyCode::Char('q') => Next::do_async(Terminate),
                _ => {
                    // TODO: Actions
                    Next::do_sync(Render)
                }
            },
            _ => Next::do_sync(Render),
        };
        ctx.do_next(next_state);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<UiEvent> for AppTui {
    async fn handle(&mut self, event: UiEvent, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            UiEvent::SetState { peers } => {
                self.state.peers.set_state(peers);
            }
            UiEvent::StateChanged => {}
        }
        let next_state = Next::do_sync(Render);
        ctx.do_next(next_state);
        Ok(())
    }
}

struct Render;

impl DoSync<Render> for AppTui {
    fn once(&mut self, _: &mut Render) -> Result<Next<Self>> {
        let terminal = self.terminal.get_mut()?;
        terminal.draw(|frame| self.state.render(frame))?;
        Ok(Next::events())
    }
}

struct Terminate;

#[async_trait]
impl DoAsync<Terminate> for AppTui {
    async fn handle(&mut self, _: Terminate, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ratatui::try_restore()?;
        Ok(Next::done())
    }
}
