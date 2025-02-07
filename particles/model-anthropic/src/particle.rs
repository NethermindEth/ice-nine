use crate::config::AnthropicConfig;
use crate::convert;
use anthropic_sdk::Client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use crb::superagent::{Entry, OnRequest};
use ice9_core::{
    ConfigSegmentUpdates, Model, Particle, SubstanceBond, SubstanceLinks, ToolingChatRequest,
    ToolingChatResponse, UpdateConfig,
};
use serde_json::json;

pub struct AnthropicParticle {
    substance: SubstanceLinks,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
    client: Slot<Client>,
}

impl Model for AnthropicParticle {}

impl Particle for AnthropicParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            config_updates: None,
            bond: Slot::empty(),
            client: Slot::empty(),
        }
    }
}

impl Agent for AnthropicParticle {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for AnthropicParticle {
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
impl UpdateConfig<AnthropicConfig> for AnthropicParticle {
    async fn update_config(
        &mut self,
        config: AnthropicConfig,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        if self.client.is_filled() {
            self.client.take()?;
        }
        let client = config.extract()?;
        self.client.fill(client)?;
        Ok(())
    }
}

#[async_trait]
impl OnRequest<ToolingChatRequest> for AnthropicParticle {
    async fn on_request(
        &mut self,
        request: ToolingChatRequest,
        _ctx: &mut Context<Self>,
    ) -> Result<ToolingChatResponse> {
        let client = self.client.take()?;

        let messages: Vec<_> = request.messages.into_iter().map(convert::message).collect();
        let config = self.substance.config::<AnthropicConfig>().await?;

        let anthropic_request = client
            .model(config.model.as_str())
            .messages(&serde_json::to_value(messages)?)
            .max_tokens(config.max_tokens)
            .build()?;

        let mut combined_response = String::new();
        let result = anthropic_request
            .execute(|text| {
                combined_response.push_str(&text);
                async {}
            })
            .await?;
        let client = config.extract()?;
        self.client.fill(client)?;
        let response_message = convert::choice(&json!({
            "role": "assistant",
            "content": combined_response,
        }))
        .ok_or_else(|| anyhow!("Failed to build response"))?;

        let messages = vec![response_message];
        let response = ToolingChatResponse { messages };
        Ok(response)
    }
}
