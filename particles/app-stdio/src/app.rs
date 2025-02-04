use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::time::{Duration, Instant};
use crb::core::Slot;
use crb::superagent::{Interval, StreamSession, Supervisor};
use ice9_core::{Particle, SubstanceLinks};
use ice_nine_plugin_control_chat::{Chat, ChatEvent};
use rustyline::{error::ReadlineError, DefaultEditor};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};
use ui9_dui::{State, Sub, SubEvent};

static RATE: u64 = 250;

pub struct StdioApp {
    editor: Slot<DefaultEditor>,
    stdout: Stdout,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,

    started: Instant,
    thinking: Option<String>,
    interval: Interval<Tick>,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            editor: Slot::empty(),
            stdout: io::stdout(),
            chat: Sub::unified(None),
            state: None,
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

impl StdioApp {
    async fn write(&mut self, text: &str) -> Result<()> {
        self.stdout.write_all(text.as_ref()).await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn writeln(&mut self, text: &str) -> Result<()> {
        self.stdout.write_all(text.as_ref()).await?;
        self.stdout.write_all(b"\n").await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn start_thinking(&mut self, reason: &str) -> Result<()> {
        self.thinking = Some(reason.into());
        Ok(())
    }

    async fn stop_thinking(&mut self) -> Result<()> {
        self.thinking.take();
        self.clear_line().await?;
        Ok(())
    }

    async fn clear_line(&mut self) -> Result<()> {
        self.stdout.write_all(b"\r\x1b[2K").await?;
        self.stdout.flush().await?;
        Ok(())
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for StdioApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let editor = DefaultEditor::new()?;
        self.editor.fill(editor)?;

        self.interval.add_listener(&ctx);
        self.interval.start();

        self.start_thinking("Loading the state...").await?;

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
        let spinner_chars = ['|', '/', '-', '\\'];
        let idx = self.started.elapsed().as_millis() as u64 / RATE % spinner_chars.len() as u64;
        let mut status = String::new();
        if let Some(reason) = self.thinking.as_ref() {
            let current_char = spinner_chars[idx as usize];
            status.push(current_char);
            status.push_str(" ");
            status.push_str(reason);
            // TODO: add dots
        }
        if !status.is_empty() {
            self.clear_line().await?;
            self.write(&status).await?;
        }
        Ok(())
    }
}

struct Asking;

#[async_trait]
impl DoAsync<Asking> for StdioApp {
    async fn handle(&mut self, _: Asking, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let rl = self.editor.get_mut()?;
        let readline = rl.readline(">> ");
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
