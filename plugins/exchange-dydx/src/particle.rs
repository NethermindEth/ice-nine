use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Duty, Next};
use ice_nine_core::{Particle, ParticleSetup, SubstanceLinks};

pub struct DyDxParticle {
    substance: SubstanceLinks,
}

impl Particle for DyDxParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup.links,
        }
    }
}

impl Agent for DyDxParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
    }
}

struct Configure;

#[async_trait]
impl Duty<Configure> for DyDxParticle {
    async fn handle(&mut self, _: Configure, _ctx: &mut Self::Context) -> Result<Next<Self>> {
        let config: DyDxConfig = self.substance.config().await?;
        Ok(Next::events())
    }
}

/*
#[async_trait]
impl OnRequest<()> for DyDxParticle {
    async fn on_request(
        &mut self,
        msg: (),
        _: &mut Self::Context,
    ) -> Result<()> {
        Ok(())
    }
}
*/
