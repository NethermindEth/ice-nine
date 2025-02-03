use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use ice9_core::{Particle, SubstanceLinks};
use ice_nine_plugin_control_chat::{Chat, ChatEvent};
use rustyline::{error::ReadlineError, DefaultEditor};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};
use ui9_dui::{State, Sub, SubEvent};

pub struct LineApp {
    editor: Slot<DefaultEditor>,
    stdout: Stdout,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,
    thinking: bool,
}

impl Particle for LineApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            editor: Slot::empty(),
            stdout: io::stdout(),
            chat: Sub::unified(None),
            state: None,
            thinking: false,
        }
    }
}

impl Supervisor for LineApp {
    type GroupBy = ();
}

impl Agent for LineApp {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl LineApp {
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

    async fn clear_line(&mut self) -> Result<()> {
        self.stdout.write_all(b"\r\x1b[2K").await?;
        self.stdout.flush().await?;
        Ok(())
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for LineApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let editor = DefaultEditor::new()?;
        self.editor.fill(editor)?;

        // TODO: Use `MultiplexSession` instead of supervisor
        let events = self.chat.events()?;
        ctx.assign(events, (), ());
        Ok(Next::events())
    }
}

struct Asking;

#[async_trait]
impl DoAsync<Asking> for LineApp {
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
impl OnEvent<SubEvent<Chat>> for LineApp {
    async fn handle(&mut self, event: SubEvent<Chat>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                self.writeln("Loading the state...").await?;
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
        if let Some(reason) = reason {
            // TODO: Spawn a timer
        } else {
            ctx.do_next(Next::do_async(Asking));
        }
        Ok(())
    }
}
