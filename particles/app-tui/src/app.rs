use crate::events::EventsDrainer;
use crate::state::AppState;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{
    Agent, AgentSession, Context, DoAsync, DoSync, ManagedContext, Next, OnEvent, RunAgent,
};
use crb::core::Slot;
use crb::runtime::InterruptionLevel;
use crb::superagent::{Interval, StreamSession, Supervisor, SupervisorSession, Tick};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use n9_core::{Particle, SubstanceLinks};
use ratatui::DefaultTerminal;

pub struct TuiApp {
    substance: SubstanceLinks,
    terminal: Slot<DefaultTerminal>,
    state: AppState,
    interval: Interval,
}

impl Particle for TuiApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            terminal: Slot::empty(),
            state: AppState::new(),
            interval: Interval::new(),
        }
    }
}

impl Supervisor for TuiApp {
    type BasedOn = StreamSession<Self>;
    type GroupBy = ();
}

impl Agent for TuiApp {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for TuiApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let terminal = ratatui::try_init()?;
        self.terminal.fill(terminal)?;

        // TODO: Use a drainer from CRB
        let drainer = EventsDrainer::new(&ctx);
        let mut runtime = RunAgent::new(drainer);
        runtime.level = InterruptionLevel::ABORT;
        ctx.spawn_runtime(runtime, ());

        self.interval.set_interval_ms(200)?;
        ctx.consume(self.interval.events()?);

        Ok(Next::do_sync(Render))
    }
}

#[async_trait]
impl OnEvent<Event> for TuiApp {
    async fn handle(&mut self, event: Event, ctx: &mut Context<Self>) -> Result<()> {
        let mut next_state = Next::do_sync(Render);
        match event {
            Event::Key(event) => {
                self.state.handle(event);
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    match event.code {
                        KeyCode::Char('q') | KeyCode::Char('w') => {
                            next_state = Next::do_async(Terminate);
                        }
                        _ => {
                            // TODO: Actions
                        }
                    }
                }
            }
            _ => {}
        }
        ctx.do_next(next_state);
        Ok(())
    }
}

struct Render;

impl DoSync<Render> for TuiApp {
    fn once(&mut self, _: &mut Render) -> Result<Next<Self>> {
        let terminal = self.terminal.get_mut()?;
        terminal.draw(|frame| self.state.render(frame))?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Tick> for TuiApp {
    async fn handle(&mut self, _: Tick, ctx: &mut Context<Self>) -> Result<()> {
        ctx.do_next(Next::do_sync(Render));
        Ok(())
    }
}

struct Terminate;

#[async_trait]
impl DoAsync<Terminate> for TuiApp {
    async fn handle(&mut self, _: Terminate, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.shutdown();
        self.substance.substance.interrupt()?;
        ratatui::try_restore()?;
        Ok(Next::done())
    }
}
