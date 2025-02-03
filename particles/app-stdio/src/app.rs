use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{Supervisor, SupervisorSession};
use ice9_core::{Particle, SubstanceLinks};
use ice_nine_plugin_control_chat::{Chat, ChatEvent};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};
use ui9_dui::{State, Sub, SubEvent};

pub struct StdioApp {
    stdin: Lines<BufReader<Stdin>>,
    stdout: Stdout,
    chat: Sub<Chat>,
    state: Option<State<Chat>>,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let lines = reader.lines();
        Self {
            stdin: lines,
            stdout: io::stdout(),
            chat: Sub::unified(None),
            state: None,
        }
    }
}

impl Supervisor for StdioApp {
    type GroupBy = ();
}

impl Agent for StdioApp {
    type Context = SupervisorSession<Self>;

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
        self.stdout.write_all("\n".as_ref()).await?;
        self.stdout.flush().await?;
        Ok(())
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for StdioApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // TODO: Use `MultiplexSession` instead of supervisor
        let events = self.chat.events()?;
        ctx.assign(events, (), ());
        Ok(Next::events())
    }
}

struct Asking;

#[async_trait]
impl DoAsync<Asking> for StdioApp {
    async fn handle(&mut self, _: Asking, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.write(">>> ").await?;
        // TODO: Add waiting timeout
        let line = self
            .stdin
            .next_line()
            .await?
            .ok_or_else(|| anyhow!("End of stdin"))?;
        self.chat.request(line);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<SubEvent<Chat>> for StdioApp {
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
                ChatEvent::Add { message } => {
                    self.writeln(&message).await?;
                }
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
            self.write("Thinking: ").await?;
            self.writeln(&reason).await?;
        } else {
            ctx.do_next(Next::do_async(Asking));
        }
        Ok(())
    }
}
