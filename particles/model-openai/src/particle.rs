use crate::config::{Client, OpenAIConfig};
use crate::convert;
use anyhow::Result;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use crb::superagent::{Entry, OnRequest};
use n9_core::{
    ConfigSegmentUpdates, Model, Particle, SubstanceBond, SubstanceLinks, ToolingChatRequest,
    ToolingChatResponse, UpdateConfig,
};
use ui9_dui::Operation;

pub struct OpenAIParticle {
    substance: SubstanceLinks,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
    client: Slot<Client>,
}

impl Model for OpenAIParticle {}

impl Particle for OpenAIParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            config_updates: None,
            bond: Slot::empty(),
            client: Slot::empty(),
        }
    }
}

impl Agent for OpenAIParticle {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for OpenAIParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);

        let (config, entry) = bond.live_config_updates().await?;
        self.config_updates = Some(entry);
        self.update_config(config, ctx).await?;

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

        let op = Operation::start("Configuring OpenAI");
        let client = Client::with_config(config.extract());
        let _models = client.models().list().await?; // An alternative to ping
        self.client.fill(client)?;
        op.end("OpenAI configured");
        Ok(())
    }
}

#[async_trait]
impl OnRequest<ToolingChatRequest> for OpenAIParticle {
    async fn on_request(
        &mut self,
        request: ToolingChatRequest,
        _: &mut Context<Self>,
    ) -> Result<ToolingChatResponse> {
        let op = Operation::start("Sending a request to OpenAI");
        let client = self.client.get_mut()?;
        // TODO: Sequental, but could be executed in the reactor
        let messages: Vec<_> = request.messages.into_iter().map(convert::message).collect();
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
        let response = ToolingChatResponse { messages };
        op.end("A request to OpenAI completed");
        Ok(response)
    }
}
