use crate::config::StdioConfig;
use crate::drainer::{Line, ReadLine, StdinDrainer};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{
    Entry, FetchError, InteractExt, Interval, OnResponse, Supervisor, SupervisorSession,
};
use ice9_core::{ConfigSegmentUpdates, Particle, SubstanceBond, SubstanceLinks, UpdateConfig};
use tokio::io::{self, AsyncWriteExt, Stdout};

pub struct StdioParticle {
    substance: SubstanceLinks,
    stdout: Stdout,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
    drainer: Slot<Address<StdinDrainer>>,
    thinking_interval: Interval<Tick>,
    config: StdioConfig,
}

impl Particle for StdioParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance: substance,
            stdout: io::stdout(),
            config_updates: None,
            bond: Slot::empty(),
            drainer: Slot::empty(),
            thinking_interval: Interval::default(),
            config: StdioConfig::default(),
        }
    }
}

impl Supervisor for StdioParticle {
    type GroupBy = ();
}

impl Agent for StdioParticle {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for StdioParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.thinking_interval.add_listener(&ctx);
        // TODO: Dry configs
        let mut bond = self.substance.bond(&ctx);
        let (config, entry) = bond.live_config_updates().await?;
        // TODO: Entry must be optional
        // + method to store/provide defaults
        self.config_updates = Some(entry);
        self.update_config(config, ctx).await?;
        self.bond.fill(bond)?;

        let drainer = StdinDrainer::new();
        let (addr, _) = ctx.spawn_agent(drainer, ());
        self.drainer.fill(addr)?;

        self.user_prompt(ctx).await?;
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
        let _line = line?.ok_or_else(|| anyhow!("Stdin is closed"))?;
        // TODO: Call the chat
        self.start_thinking(ctx).await?;
        Ok(())
    }
}

impl StdioParticle {
    async fn user_prompt(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        self.stdout.write_all(b"User: ").await?;
        self.stdout.flush().await?;
        let task = self.drainer.get()?.interact(ReadLine);
        ctx.assign(task, (), ());
        Ok(())
    }

    async fn start_thinking(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        self.stdout.write_all(b"Thinking").await?;
        self.stdout.flush().await?;
        // TODO: Start an interval

        self.thinking_interval.start();
        Ok(())
    }

    async fn progress_thinking(&mut self) -> Result<()> {
        let loader = self.config.loader.as_ref();
        self.stdout.write_all(loader).await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn stop_thinking(&mut self) -> Result<()> {
        self.thinking_interval.stop();
        Ok(())
    }
}

#[async_trait]
impl UpdateConfig<StdioConfig> for StdioParticle {
    async fn update_config(&mut self, config: StdioConfig, _ctx: &mut Context<Self>) -> Result<()> {
        self.config = config;
        Ok(())
    }
}

#[derive(Clone, Default)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for StdioParticle {
    async fn handle(&mut self, _: Tick, _ctx: &mut Context<Self>) -> Result<()> {
        self.progress_thinking().await?;
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
