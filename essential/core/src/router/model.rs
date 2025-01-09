use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt};
use crb::superagent::{OnRequest, Request};
use derive_more::{Deref, DerefMut};

pub trait Model: OnRequest<ChatRequest> {}

pub enum Role {
    Developer,
    User,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Default)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
}

impl Request for ChatRequest {
    type Response = ChatResponse;
}

#[derive(Default)]
pub struct ChatResponse {
    pub messages: Vec<Message>,
}

#[derive(Deref, DerefMut)]
pub struct ModelClient {
    address: Box<dyn ModelAddress>,
}

#[async_trait]
pub trait ModelAddress: Send {
    async fn chat(&mut self, request: ChatRequest) -> Result<ChatResponse>;
}

#[async_trait]
impl<M> ModelAddress for Address<M>
where
    M: Model,
{
    async fn chat(&mut self, request: ChatRequest) -> Result<ChatResponse> {
        let response = self.interact(request)?.await?;
        Ok(response)
    }
}
