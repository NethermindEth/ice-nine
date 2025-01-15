use crate::config::{Client, OpenAIConfig};
use crate::convert;
use anyhow::Result;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next};
use crb::core::Slot;
use crb::superagent::OnRequest;
use ice_nine_core::{
    Model, Particle, ParticleSetup, SubstanceBond, ToolingChatRequest, ToolingChatResponse,
    UpdateConfig,
};

pub struct OpenAIParticle {
    substance: ParticleSetup,
    bond: Slot<SubstanceBond<Self>>,

    client: Slot<Client>,
}

impl Model for OpenAIParticle {}

impl Particle for OpenAIParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup,
            bond: Slot::empty(),
            client: Slot::empty(),
        }
    }
}

impl Agent for OpenAIParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for OpenAIParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&*ctx);
        bond.live_config_updates().await?;
        bond.add_model()?;
        self.bond.fill(bond)?;

        Ok(Next::events())
    }
}

#[async_trait]
impl UpdateConfig<OpenAIConfig> for OpenAIParticle {
    async fn update_config(
        &mut self,
        config: OpenAIConfig,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        if self.client.is_filled() {
            self.client.take()?;
        }

        let client = Client::with_config(config.0);
        let _models = client.models().list().await?; // An alternative to ping
        self.client.fill(client)?;
        Ok(())
    }
}

#[async_trait]
impl OnRequest<ToolingChatRequest> for OpenAIParticle {
    async fn on_request(
        &mut self,
        msg: ToolingChatRequest,
        _: &mut Context<Self>,
    ) -> Result<ToolingChatResponse> {
        let client = self.client.get_mut()?;
        // TODO: Sequental, but could be executed in the reactor
        let messages: Vec<_> = msg.messages.into_iter().map(convert::message).collect();
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o")
            .messages(messages)
            .build()?;
        let response = client.chat().create(request).await?;
        let messages = response
            .choices
            .into_iter()
            .filter_map(convert::choice)
            .collect();
        Ok(ToolingChatResponse { messages })
    }
}
