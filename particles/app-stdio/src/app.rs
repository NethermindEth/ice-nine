use crate::editor::IoControl;
use crate::queue::Queue;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use colored::Colorize;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::time::{Duration, Instant};
use crb::core::Slot;
use crb::superagent::{Interval, StreamSession, Supervisor};
use ice9_core::{Particle, SubstanceLinks};
use ice_nine_plugin_control_chat::{Chat, ChatEvent};
use rustyline::{
    error::ReadlineError,
    validate::{ValidationContext, ValidationResult, Validator},
    Cmd, Config, DefaultEditor, Editor, Event, KeyCode, KeyEvent, Modifiers,
};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};
use ui9_dui::{State, Sub, SubEvent};

static RATE: u64 = 200;

pub struct StdioApp {
    io_control: Slot<IoControl>,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,

    queue: Queue,
    started: Instant,
    thinking: Option<String>,
    interval: Interval<Tick>,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            io_control: Slot::empty(),
            chat: Sub::unified(None),
            state: None,

            queue: Queue::new(),
            started: Instant::now(),
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

        self.interval.add_listener(&ctx);
        self.interval.start();

        self.queue.add_message("Loading the state...");
        io_control.start_thinking().await?;

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
        let spinner_chars = ['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾'];
        let idx = self.started.elapsed().as_millis() as u64 / RATE % spinner_chars.len() as u64;
        if let Some(reason) = self.queue.pick_next() {
            let mut status = String::new();
            let current_char = spinner_chars[idx as usize];
            status.push_str(&current_char.to_string().green().to_string());
            status.push_str(" ");
            status.push_str(&reason);
            io_control.clear_line().await?;
            io_control.write(&status).await?;
        }
        Ok(())
    }
}

struct Asking;

#[async_trait]
impl DoAsync<Asking> for StdioApp {
    async fn handle(&mut self, _: Asking, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let io_control = self.io_control.get_mut()?;
        let readline = io_control.readline(">> ");
        match readline {
            Ok(line) => {
                // TODO: Actions must be interactive! Do await!
                self.chat.request(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
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
                ChatEvent::Add { message } => {}
                ChatEvent::SetThinking { .. } => {}
            },
            SubEvent::Lost => {
                self.state.take();
            }
        }
        let reason = self
            .state
            .as_ref()
            .map(State::borrow)
            .and_then(|state| state.thinking.clone());
        /*
        if let Some(reason) = reason {
            // TODO: Spawn a timer
        } else {
            ctx.do_next(Next::do_async(Asking));
        }
        */
        Ok(())
    }
}
