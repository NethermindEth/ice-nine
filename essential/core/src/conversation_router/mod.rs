use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Next, OnEvent, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};

#[derive(Deref, DerefMut, From, Clone)]
pub struct ConversationRouterClient {
    address: Address<ConversationRouter>,
}

pub struct ConversationRouter {}

impl ConversationRouter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for ConversationRouter {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

pub struct AddModel {}

#[async_trait]
impl OnEvent<AddModel> for ConversationRouter {
    type Error = Error;

    async fn handle(&mut self, _: AddModel, _ctx: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}

pub struct TextRequest {
    text: String,
}

impl Request for TextRequest {
    type Response = ();
}

#[async_trait]
impl OnRequest<TextRequest> for ConversationRouter {
    async fn on_request(&mut self, _lookup: TextRequest, _: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}
