use crate::config::{Client, OpenAIConfig};
use crate::convert;
use anyhow::Result;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, InContext, Next};
use crb::core::types::Slot;
use crb::superagent::OnRequest;
use ice_nine_core::{
    Model, Particle, ParticleSetup, SubstanceLinks, ToolingChatRequest, ToolingChatResponse,
};

pub struct OpenAIParticle {
    substance: SubstanceLinks,
    client: Slot<Client>,
}

impl Model for OpenAIParticle {}

impl Particle for OpenAIParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup.links,
            client: Slot::empty(),
        }
    }
}

impl Agent for OpenAIParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::in_context(Configure)
    }
}

struct Configure;

#[async_trait]
impl InContext<Configure> for OpenAIParticle {
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        println!("Configuring...");

        let config: OpenAIConfig = self.substance.config().await?;
        let client = Client::with_config(config.0);
        let _models = client.models().list().await?; // An alternative to ping
        self.client.fill(client)?;

        let address = ctx.address().clone();
        self.substance.router.add_model(address)?;

        Ok(Next::events())
    }
}

#[async_trait]
impl OnRequest<ToolingChatRequest> for OpenAIParticle {
    async fn on_request(
        &mut self,
        msg: ToolingChatRequest,
        _: &mut Self::Context,
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
