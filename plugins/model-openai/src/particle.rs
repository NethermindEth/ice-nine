use crate::convert;
use anyhow::Result;
use async_openai::types::CreateChatCompletionRequest;
use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next};
use crb::core::types::Slot;
use crb::superagent::OnRequest;
use ice_nine_core::{ChatRequest, ChatResponse, KeeperClient, Model, Particle, ParticleSetup};

const NAMESPACE: &'static str = "OPENAI";

type Client = OpenAIClient<OpenAIConfig>;

pub struct OpenAIParticle {
    keeper: KeeperClient,
    client: Slot<Client>,
}

impl Model for OpenAIParticle {}

impl Particle for OpenAIParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            keeper: setup.keeper,
            client: Slot::empty(),
        }
    }
}

impl Agent for OpenAIParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Configure)
    }
}

struct Configure;

#[async_trait]
impl DoAsync<Configure> for OpenAIParticle {
    async fn once(&mut self, _: &mut Configure) -> Result<Next<Self>> {
        println!("Configuring...");
        let config: OpenAIConfig = self.keeper.get_config(NAMESPACE).await?;
        let client = Client::with_config(config);
        let _models = client.models().list().await?; // An alternative to ping
        self.client.fill(client)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnRequest<ChatRequest> for OpenAIParticle {
    async fn on_request(
        &mut self,
        msg: ChatRequest,
        _: &mut Self::Context,
    ) -> Result<ChatResponse> {
        let client = self.client.get_mut()?;
        // TODO: Sequental, but could be executed in the reactor
        let mut request = CreateChatCompletionRequest::default();
        let messages = msg.messages.into_iter().map(convert::message);
        request.messages.extend(messages);
        let response = client.chat().create(request).await?;
        let messages = response
            .choices
            .into_iter()
            .filter_map(convert::choice)
            .collect();
        Ok(ChatResponse { messages })
    }
}
