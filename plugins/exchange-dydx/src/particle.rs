use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next};
use crb::superagent::{OnRequest, Request, Supervisor, SupervisorSession};
use ice_nine_core::{Particle, ParticleSetup, SubstanceLinks, Tool, ToolMeta, ToolResponse};
use serde::Deserialize;

pub struct DyDxParticle {
    substance: ParticleSetup,
}

impl Supervisor for DyDxParticle {
    type GroupBy = ();
}

impl Tool for DyDxParticle {}

impl Particle for DyDxParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self { substance: setup }
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
        */
        let meta = todo!();
        self.substance.router.add_tool::<_, Price>(address, meta)?;
        self.substance.router.add_tool::<_, Trade>(address, meta)?;
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

#[derive(Deserialize)]
pub struct Trade {
    ticker: String,
}

impl Request for Trade {
    type Response = ToolResponse;
}

#[async_trait]
impl OnRequest<Trade> for DyDxParticle {
    async fn on_request(&mut self, msg: Trade, _: &mut Self::Context) -> Result<ToolResponse> {
        todo!()
    }
}
