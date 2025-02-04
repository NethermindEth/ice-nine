use crate::editor::{IoControl, RATE};
use crate::queue::Queue;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use colored::Colorize;
use crb::agent::{Agent, Context, DoAsync, DoSync, Next, OnEvent};
use crb::core::time::{Duration, Instant};
use crb::core::Slot;
use crb::superagent::{Interval, StreamSession, Supervisor};
use ice9_core::{ChatRequest, Particle, SubstanceLinks};
use ice_nine_plugin_control_chat::{Chat, ChatEvent};
use rustyline::{
    error::ReadlineError,
    validate::{ValidationContext, ValidationResult, Validator},
    Cmd, Config, DefaultEditor, Editor, Event, KeyCode, KeyEvent, Modifiers,
};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};
use ui9_dui::{State, Sub, SubEvent};

pub struct StdioApp {
    substance: SubstanceLinks,
    io_control: Slot<IoControl>,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,

    queue: Queue,
    thinking: Option<String>,
    interval: Interval<Tick>,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            io_control: Slot::empty(),
            chat: Sub::unified(None),
            state: None,

            queue: Queue::new(),
            thinking: None,
            interval: Interval::new(Tick, Duration::from_millis(RATE)),
        }
    }
}

impl Supervisor for StdioApp {
    type GroupBy = ();
}

impl Agent for StdioApp {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for StdioApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.io_control.fill(IoControl::new()?)?;
        let io_control = self.io_control.get_mut()?;
        io_control
            .writeln(&"Nine - Stdio Client".green().to_string())
            .await?;

        self.interval.add_listener(&ctx);

        self.queue.add_message("Loading the state...");
        self.interval.start();

        // TODO: Use `MultiplexSession` instead of supervisor
        let events = self.chat.events()?.into_events_stream();
        ctx.consume(events);
        Ok(Next::events())
    }
}

#[derive(Clone)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for StdioApp {
    async fn handle(&mut self, _: Tick, ctx: &mut Context<Self>) -> Result<()> {
        let io_control = self.io_control.get_mut()?;
        if let Some(reason) = self.queue.pick_next() {
            io_control.render_progress(reason).await?;
        } else {
            self.interval.stop();
            io_control.clear_line().await?;
            ctx.do_next(Next::do_sync(Prompt));
        }
        Ok(())
    }
}

struct Prompt;

impl DoSync<Prompt> for StdioApp {
    fn once(&mut self, _: &mut Prompt) -> Result<Next<Self>> {
        let io_control = self.io_control.get_mut()?;
        let prompt = io_control.prompt()?;
        Ok(Next::do_async(Asking { prompt }))
    }
}

struct Asking {
    prompt: String,
}

#[async_trait]
impl DoAsync<Asking> for StdioApp {
    async fn handle(&mut self, msg: Asking, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let io_control = self.io_control.get_mut()?;
        io_control.clear_line().await?;
        self.chat.request(msg.prompt);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<SubEvent<Chat>> for StdioApp {
    async fn handle(&mut self, event: SubEvent<Chat>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                self.queue.add_message("Chat state has loaded");
                {
                    let state_ref = state.borrow();
                    for message in &state_ref.messages {}
                }
                self.state = Some(state);
            }
            SubEvent::Event(event) => match event {
                ChatEvent::Add { message } => {
                    let io_control = self.io_control.get_mut()?;
                    io_control.writeln(&message).await?;
                }
                ChatEvent::SetThinking { flag } => {
                    if flag {
                        self.queue.add_message("Thinking...");
                        self.interval.start();
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
