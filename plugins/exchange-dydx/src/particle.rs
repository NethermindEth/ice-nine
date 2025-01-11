use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next};
use crb::superagent::{OnRequest, Request};
use ice_nine_core::{Particle, ParticleSetup, SubstanceLinks, Tool, ToolMeta, ToolResponse};
use serde::Deserialize;

pub struct DyDxParticle {
    substance: SubstanceLinks,
}

impl Tool for DyDxParticle {
    type Request = Price;
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
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let config: DyDxConfig = self.substance.config().await?;
        let address = ctx.address().clone();
        /*
        let meta = ToolMeta {
        };
        */
        let meta = todo!();
        self.substance.router.add_tool(address, meta)?;
        Ok(Next::events())
    }
}

#[derive(Deserialize)]
pub struct Price {
    ticker: String,
}

impl Request for Price {
    type Response = ToolResponse;
}

#[async_trait]
impl OnRequest<Price> for DyDxParticle {
    async fn on_request(&mut self, msg: Price, _: &mut Self::Context) -> Result<ToolResponse> {
        todo!()
    }
}
