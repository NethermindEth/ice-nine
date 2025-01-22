use crate::config::{Client, OpenAIConfig};
use crate::convert;
use anyhow::Result;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next};
use crb::core::Slot;
use crb::superagent::{Entry, OnRequest};
use ice9_core::{
    ConfigSegmentUpdates, Model, Particle, SubstanceBond, SubstanceLinks, ToolingChatRequest,
    ToolingChatResponse, UpdateConfig,
};
use ui9::names::Fqn;
use ui9_tracers::MessageTracer;

pub struct OpenAIParticle {
    substance: SubstanceLinks,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
    client: Slot<Client>,
    message_tracer: MessageTracer,
}

impl Model for OpenAIParticle {}

impl Particle for OpenAIParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        let fqn = Fqn::root("openai");
        Self {
            substance,
            config_updates: None,
            bond: Slot::empty(),
            client: Slot::empty(),
            message_tracer: MessageTracer::new(fqn),
        }
    }
}

impl Agent for OpenAIParticle {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for OpenAIParticle {
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

        let client = Client::with_config(config.extract());
        let _models = client.models().list().await?; // An alternative to ping
        self.client.fill(client)?;
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
        let client = self.client.get_mut()?;
        self.message_tracer.add_message(&request.squash());
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
        self.message_tracer.add_message(&response.squash());
        Ok(response)
    }
}
