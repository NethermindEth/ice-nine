use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, AgentSession, DoAsync, Next, OnRequest, Request};
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

pub struct TextRequest {
    text: String,
}

impl Request for TextRequest {
    type Response = ();
}

#[async_trait]
impl OnRequest<TextRequest> for ConversationRouter {
    async fn on_request(&mut self, lookup: TextRequest, _: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}
