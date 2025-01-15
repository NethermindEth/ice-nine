use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Duty, Next};
use ice_nine_core::{Particle, ParticleSetup};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin, Stdout};

pub struct StdioParticle {
    lines: Lines<BufReader<Stdin>>,
    stdout: Stdout,
}

impl Particle for StdioParticle {
    fn construct(setup: ParticleSetup) -> Self {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        Self {
            lines: reader.lines(),
            stdout: io::stdout(),
        }
    }
}

impl Agent for StdioParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for StdioParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // TODO: Connect to the substance
        Ok(Next::do_async(ReadLine))
    }
}

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
