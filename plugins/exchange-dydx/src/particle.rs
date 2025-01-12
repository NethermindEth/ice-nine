use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use ice_nine_core::{Particle, ParticleSetup, SubstanceBond, Tool};
use serde::Deserialize;

pub struct DyDxParticle {
    substance: ParticleSetup,
    bond: Slot<SubstanceBond<Self>>,
}

impl Supervisor for DyDxParticle {
    type GroupBy = ();
}

impl Particle for DyDxParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup,
            bond: Slot::empty(),
        }
    }
}

impl Agent for DyDxParticle {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
    }
}

struct Configure;

#[async_trait]
impl Duty<Configure> for DyDxParticle {
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let config: DyDxConfig = self.substance.config().await?;
        let address = ctx.address().clone();
        let mut bond = self.substance.bond(address);
        bond.add_tool::<Price>(self).await?;
        // bond.add_tool::<Trade>(self)?;
        self.bond.fill(bond)?;
        Ok(Next::events())
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
