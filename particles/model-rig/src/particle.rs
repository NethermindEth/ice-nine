use crate::proxy::VCompletionModel;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use crb::superagent::OnRequest;
use n9_core::{
    Model, Particle, SubstanceBond, SubstanceLinks, ToolingChatRequest, ToolingChatResponse,
};

pub struct RigModelParticle {
    substance: SubstanceLinks,
    bond: Slot<SubstanceBond<Self>>,
    model: Option<Box<dyn VCompletionModel>>,
}

impl Model for RigModelParticle {}

impl Particle for RigModelParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            bond: Slot::empty(),
            model: None,
        }
    }
}

impl Agent for RigModelParticle {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for RigModelParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        bond.add_model()?;
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnRequest<ToolingChatRequest> for RigModelParticle {
    async fn on_request(
        &mut self,
        request: ToolingChatRequest,
        _: &mut Context<Self>,
    ) -> Result<ToolingChatResponse> {
        Err(anyhow!("Rig models are not implemented yet"))
    }
}
