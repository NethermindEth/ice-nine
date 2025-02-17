use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use crb::superagent::{Entry, Supervisor, SupervisorSession};
use n9_core::{ConfigSegmentUpdates, Particle, SubstanceBond, SubstanceLinks, Tool, UpdateConfig};
use serde::Deserialize;

pub struct DyDxParticle {
    substance: SubstanceLinks,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
}

impl Supervisor for DyDxParticle {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Particle for DyDxParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            config_updates: None,
            bond: Slot::empty(),
        }
    }
}

impl Agent for DyDxParticle {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for DyDxParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);

        let (config, entry) = bond.live_config_updates().await?;
        self.config_updates = Some(entry);
        self.update_config(config, ctx).await?;

        bond.add_tool::<Price>(self).await?;
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl UpdateConfig<DyDxConfig> for DyDxParticle {
    async fn update_config(&mut self, _: DyDxConfig, _ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Price {
    ticker: String,
}

impl Tool<Price> for DyDxParticle {
    fn name(&self) -> String {
        "dydx_price".into()
    }

    fn description(&self) -> Option<String> {
        Some(
            "Connects to the dYdX exchange AI interface to retrieve real-time price data for
        any token available on the platform. The function accepts the token symbol as input
        and returns the current price along with relevant market details."
                .into(),
        )
    }
}

#[derive(Deserialize)]
pub struct Trade {
    ticker: String,
}

impl Tool<Trade> for DyDxParticle {}
