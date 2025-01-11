use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next};
use crb::superagent::{OnRequest, Request, Supervisor, SupervisorSession};
use ice_nine_core::{Particle, ParticleSetup, SubstanceLinks, Tool, ToolMeta, ToolResponse};
use serde::Deserialize;

pub struct DyDxParticle {
    substance: SubstanceLinks,
}

impl Supervisor for DyDxParticle {
    type GroupBy = ();
}

impl Particle for DyDxParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup.links,
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
        /*
        let meta = ToolMeta {
        };
        let meta = todo!();
        self.substance.router.add_tool(address, meta)?;
        */
        Ok(Next::events())
    }
}
