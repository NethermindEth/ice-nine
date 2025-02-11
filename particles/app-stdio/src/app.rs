use crate::console::Console;
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crb::agent::{Agent, Context, DoAsync, DoSync, ManagedContext, Next, OnEvent};
use crb::core::time::{sleep, Duration};
use crb::core::Slot;
use crb::superagent::{Interval, StreamSession, Tick};
use n9_control_chat::{Chat, ChatEvent, Role};
use n9_core::{Particle, SubstanceLinks};
use std::collections::VecDeque;
use ui9_dui::tracers::live::{Live, LiveData};
use ui9_dui::{State, Sub, SubEvent};

pub struct StdioApp {
    substance: SubstanceLinks,
    console: Slot<Console>,
    messages: VecDeque<String>,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,
    live: Sub<Live>,
    interval: Interval,
    waiting: bool,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            console: Slot::empty(),
            messages: VecDeque::new(),
            chat: Sub::local_unified(),
            state: None,
            live: Sub::local_unified(),
            interval: Interval::new(),
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
        self.console.fill(Console::new()?)?;
        let console = self.console.get_mut()?;
        console.write(&"Nine".blue().bold().to_string()).await?;
        console
            .write(
                &" | Nethermind Intelligent Nodes Environment"
                    .blue()
                    .to_string(),
            )
            .await?;
        let version = format!(" | version {}", env!("CARGO_PKG_VERSION"))
            .yellow()
            .to_string();
        console.writeln(&version).await?;

        self.add_message("Loading the state...");
        self.interval.set_interval_ms(200)?;
        ctx.consume(self.interval.events()?);
        ctx.consume(self.chat.events()?);
        ctx.consume(self.live.events()?);
        Ok(Next::events())
    }
}

struct News;

#[async_trait]
impl DoAsync<News> for StdioApp {
    async fn repeat(&mut self, _: &mut News) -> Result<Option<Next<Self>>> {
        let console = self.console.get_mut()?;
        if let Some(message) = self.messages.pop_front() {
            console.render_progress(&message).await?;
            sleep(Duration::from_millis(400)).await;
            Ok(None)
        } else {
            // Remove the progress info completely
            // and prepare to interactions
            console.clear_line().await?;
            if self.waiting {
                Ok(Some(Next::events()))
            } else {
                Ok(Some(Next::do_sync(Prompt)))
            }
        }
    }
}

struct Prompt;

impl DoSync<Prompt> for StdioApp {
    fn once(&mut self, _: &mut Prompt) -> Result<Next<Self>> {
        let console = self.console.get_mut()?;
        if let Ok(prompt) = console.prompt() {
            if !prompt.trim().is_empty() {
                self.chat.request(prompt);
                self.waiting = true;
            }
            Ok(Next::events())
        } else {
            Ok(Next::do_async(Terminate))
        }
    }
}

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

struct Terminate;

#[async_trait]
impl DoAsync<Terminate> for StdioApp {
    async fn handle(&mut self, _: Terminate, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let console = self.console.get_mut()?;
        console.writeln("Closing the session 🙌").await?;
        // To avoid entering the prompt state
        self.waiting = true;
        ctx.shutdown();
        self.substance.substance.interrupt()?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<SubEvent<Chat>> for StdioApp {
    async fn handle(&mut self, event: SubEvent<Chat>, _ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                self.add_message("Chat state has loaded");
                {
                    let state_ref = state.borrow();
                    for _message in &state_ref.messages {}
                }
                self.state = Some(state);
            }
            SubEvent::Event(event) => match event {
                ChatEvent::Add { message } => {
                    let console = self.console.get_mut()?;
                    let role = match message.role {
                        Role::Request => "👤 Request:".blue(),
                        Role::Response => "🤖 Response:".yellow(),
                    };
                    console.writeln(&role.to_string()).await?;
                    console.write_md(&message.content).await?;
                }
                ChatEvent::SetThinking { flag } => {
                    self.waiting = flag;
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
    async fn handle(&mut self, event: SubEvent<Live>, _ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                for message in state.borrow().messages.iter() {
                    self.add_message(message);
                }
            }
            SubEvent::Event(event) => match event {
                LiveData::Message(msg) => {
                    self.add_message(&msg);
                }
                _ => {}
            },
            SubEvent::Lost => {}
        }
        Ok(())
    }
}
