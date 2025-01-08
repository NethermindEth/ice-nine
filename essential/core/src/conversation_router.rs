use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut, From};
use crb::agent::{Address, AddressExt, Agent, AgentSession, DoAsync, Next, OnRequest, Request};

#[derive(Deref, DerefMut, From, Clone)]
pub struct ConversationRouterClient {
    address: Address<ConversationRouter>,
}

pub struct ConversationRouter {
}

impl ConversationRouter {
    pub fn new() -> Self {
        Self {
        }
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
