use crb::agent::{Address, AddressExt};
use crb::superagent::{Fetcher, OnRequest, Request};
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
pub struct ModelLink {
    address: Box<dyn ModelAddress>,
}

impl<M: Model> From<Address<M>> for ModelLink {
    fn from(addr: Address<M>) -> Self {
        Self {
            address: Box::new(addr),
        }
    }
}

pub trait ModelAddress: Send {
    fn chat(&mut self, request: ChatRequest) -> Fetcher<ChatRequest>;
}

impl<M: Model> ModelAddress for Address<M> {
    fn chat(&mut self, request: ChatRequest) -> Fetcher<ChatRequest> {
        self.interact(request)
    }
}
