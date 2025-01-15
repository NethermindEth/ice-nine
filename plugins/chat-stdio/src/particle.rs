use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Next, DoAsync, Duty, Context};
use ice_nine_core::{ParticleSetup, Particle};
use tokio::io::{stdin, BufReader, Stdin, AsyncBufReadExt, Lines};

pub struct StdioParticle {
    lines: Lines<BufReader<Stdin>>,
}

impl Particle for StdioParticle {
    fn construct(setup: ParticleSetup) -> Self {
        let reader = BufReader::new(stdin());
        Self {
            lines: reader.lines(),
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
        let line = self.lines.next_line().await?;
        Ok(None)
    }
}
