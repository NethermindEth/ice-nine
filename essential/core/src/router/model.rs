use crb::superagent::{OnRequest, Request};

pub trait Model: OnRequest<ChatRequest> {}

pub enum Role {
    Developer,
    User,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

pub struct ChatRequest {
    pub messages: Vec<Message>,
}

impl Request for ChatRequest {
    type Response = ChatResponse;
}

pub struct ChatResponse {
    pub messages: Vec<Message>,
}
