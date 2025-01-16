use crate::config::StdioConfig;
use crate::drainer::{Line, ReadLine, StdinDrainer};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next};
use crb::core::Slot;
use crb::superagent::{Entry, FetchError, InteractExt, OnResponse, Supervisor, SupervisorSession};
use ice_nine_core::{ConfigSegmentUpdates, Particle, ParticleSetup, SubstanceBond, UpdateConfig};
use tokio::io::{self, AsyncWriteExt, Stdout};

pub struct StdioParticle {
    substance: ParticleSetup,
    stdout: Stdout,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
    drainer: Slot<Address<StdinDrainer>>,
}

impl Particle for StdioParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup,
            stdout: io::stdout(),
            config_updates: None,
            bond: Slot::empty(),
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
        // TODO: Dry configs
        let mut bond = self.substance.bond(&*ctx);
        let (config, entry) = bond.live_config_updates().await?;
        self.config_updates = Some(entry);
        self.update_config(config, ctx).await?;
        self.bond.fill(bond)?;

        let drainer = StdinDrainer::new();
        let (addr, _) = ctx.spawn_agent(drainer, ());
        self.drainer.fill(addr)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnResponse<Line> for StdioParticle {
    async fn on_response(
        &mut self,
        line: Result<Line, FetchError>,
        _: (),
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        let line = line?.ok_or_else(|| anyhow!("Stdin is closed"))?;
        // TODO: Call the chat
        self.start_thinking(ctx).await?;
        Ok(())
    }
}

impl StdioParticle {
    async fn user_prompt(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        self.stdout.write_all(b"User: ").await?;
        self.stdout.flush().await?;
        self.drainer
            .get()?
            .interact(ReadLine)
            .forwardable()
            .forward_to(ctx, ());
        Ok(())
    }

    async fn start_thinking(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        self.stdout.write_all(b"Thinking").await?;
        self.stdout.flush().await?;
        // TODO: Start an interval
        Ok(())
    }
}

#[async_trait]
impl UpdateConfig<StdioConfig> for StdioParticle {
    async fn update_config(&mut self, config: StdioConfig, ctx: &mut Context<Self>) -> Result<()> {
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
