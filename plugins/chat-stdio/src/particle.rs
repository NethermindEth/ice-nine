use crate::drainer::{Line, StdinDrainer};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, DoAsync, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use ice_nine_core::{Particle, ParticleSetup};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};

pub struct StdioParticle {
    stdout: Stdout,
    drainer: Slot<Address<StdinDrainer>>,
}

impl Particle for StdioParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            stdout: io::stdout(),
            drainer: Slot::empty(),
        }
    }
}

impl Supervisor for StdioParticle {
    type GroupBy = ();
}

impl Agent for StdioParticle {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for StdioParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let recipient = ctx.recipient();
        let drainer = StdinDrainer::new(recipient);
        let (addr, _) = ctx.spawn_agent(drainer, ());
        self.drainer.fill(addr)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Line> for StdioParticle {
    async fn handle(&mut self, _: Line, ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}

/*
struct ReadLine;

#[async_trait]
impl DoAsync<ReadLine> for StdioParticle {
    async fn repeat(&mut self, _: &mut ReadLine) -> Result<Option<Next<Self>>> {
        let line = self.user_prompt().await?;
        self.start_thinking().await?;
        self.stop_thinking().await?;
        match line {
            Some(line) => {
                self.system_output(&line).await?;
                Ok(None)
            }
            None => Ok(Some(Next::done())),
        }
    }
}

impl StdioParticle {
    async fn user_prompt(&mut self) -> Result<Option<String>> {
        self.stdout.write_all(b"User: ").await?;
        self.stdout.flush().await?;
        let line = self.lines.next_line().await?;
        Ok(line)
    }

    async fn start_thinking(&mut self) -> Result<()> {
        self.stdout.write_all(b"Thinking").await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn stop_thinking(&mut self) -> Result<()> {
        self.stdout.write_all(b"\n").await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn system_output(&mut self, output: &str) -> Result<()> {
        self.stdout.write_all(b"System: ").await?;
        self.stdout.write_all(output.as_ref()).await?;
        self.stdout.write_all(b"\n").await?;
        self.stdout.flush().await?;
        Ok(())
    }
}
*/
