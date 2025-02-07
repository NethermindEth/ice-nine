use crate::input::{self, CtrlC};
use crate::output::{IoControl, RATE};
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use ice9_core::{Particle, SubstanceLinks};
use crb::agent::{Agent, Next, DoAsync, Context, OnEvent};
use crb::superagent::{StreamSession, Drainer, Timer};
use crb::core::Slot;
use crb::core::time::{sleep, Duration};
use n9_control_chat::{Chat, ChatEvent, Role};
use ui9_dui::{State, Sub, SubEvent};
use ui9_dui::tracers::live::Live;
use std::collections::VecDeque;

pub struct StdioApp {
    substance: SubstanceLinks,
    io_control: Slot<IoControl>,
    messages: VecDeque<String>,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,
    live: Sub<Live>,
    input: Drainer<Result<String>>,
    timer: Timer<Tick>,
    waiting: bool,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            io_control: Slot::empty(),
            messages: VecDeque::new(),
            chat: Sub::unified(None),
            state: None,
            live: Sub::unified(None),
            input: input::lines(),
            // TODO: Use interval instead
            timer: Timer::new(Tick),
            waiting: false,
        }
    }
}

impl Agent for StdioApp {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl StdioApp {
    pub fn add_message(&mut self, content: &str) {
        self.messages.push_back(content.into());
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for StdioApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.timer.add_listener(&ctx);
        self.timer.set_duration(Duration::from_millis(200));
        self.io_control.fill(IoControl::new()?)?;
        let io_control = self.io_control.get_mut()?;
        io_control.writeln(&"Nine".blue().to_string()).await?;
        self.add_message("Loading the state...");
        ctx.consume(self.chat.events()?);
        ctx.consume(self.live.events()?);
        // ctx.consume(input::lines());
        ctx.consume(input::signals());

        self.timer.restart();
        Ok(Next::events())
    }
}

struct News;

#[async_trait]
impl DoAsync<News> for StdioApp {
    async fn repeat(&mut self, _: &mut News) -> Result<Option<Next<Self>>> {
        let io_control = self.io_control.get_mut()?;
        if let Some(message) = self.messages.pop_front() {
            io_control.render_progress(&message).await?;
            sleep(Duration::from_millis(400)).await;
            Ok(None)
        } else {
            io_control.clear_line().await?;
            if self.waiting {
                self.timer.restart();
                Ok(Some(Next::events()))
            } else {
                Ok(Some(Next::do_async(Prompt)))
            }
        }
    }
}

struct Prompt;

#[async_trait]
impl DoAsync<Prompt> for StdioApp {
    async fn once(&mut self, _: &mut Prompt) -> Result<Next<Self>> {
        let io_control = self.io_control.get_mut()?;
        io_control.write(">> ").await?;
        let prompt = input::next_line().await?;
        self.chat.request(prompt);
        io_control.move_up().await?;
        io_control.clear_line().await?;
        self.waiting = true;

        self.timer.restart();
        Ok(Next::events())
    }
}

#[derive(Clone)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for StdioApp {
    async fn handle(&mut self, _: Tick, ctx: &mut Context<Self>) -> Result<()> {
        if self.waiting {
            self.add_message("Thinking...");
        }
        ctx.do_next(Next::do_async(News));
        Ok(())
    }
}

#[async_trait]
impl OnEvent<CtrlC> for StdioApp {
    async fn handle(&mut self, _: CtrlC, ctx: &mut Context<Self>) -> Result<()> {
        let io_control = self.io_control.get_mut()?;
        io_control.clear_line().await?;
        io_control.writeln("Closing the session 🙌").await?;
        self.substance.substance.interrupt()
    }
}

#[async_trait]
impl OnEvent<SubEvent<Chat>> for StdioApp {
    async fn handle(&mut self, event: SubEvent<Chat>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                self.add_message("Chat state has loaded");
                {
                    let state_ref = state.borrow();
                    for message in &state_ref.messages {}
                }
                self.state = Some(state);
            }
            SubEvent::Event(event) => match event {
                ChatEvent::Add { message } => {
                    let io_control = self.io_control.get_mut()?;
                    let role = match message.role {
                        Role::Request => "👤 Request:".blue(),
                        Role::Response => "🤖 Response:".yellow(),
                    };
                    io_control.writeln(&role.to_string()).await?;
                    io_control.write_md(&message.content).await?;
                }
                ChatEvent::SetThinking { flag } => {
                    if flag {
                        self.add_message("Thinking...");
                    } else {
                        // TODO: Add an event to request...
                    }
                }
            },
            SubEvent::Lost => {
                self.state.take();
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<SubEvent<Live>> for StdioApp {
    async fn handle(&mut self, event: SubEvent<Live>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                for message in state.borrow().messages.iter() {
                    self.add_message(message);
                }
            }
            SubEvent::Event(event) => {
                let message = String::from(event);
                self.add_message(&message);
            }
            SubEvent::Lost => {}
        }
        Ok(())
    }
}
