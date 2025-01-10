use crb::agent::{Address, AddressExt};
use crb::superagent::{Fetcher, OnRequest, Request};
use derive_more::{Deref, DerefMut};
use std::sync::Arc;

pub enum Role {
    Developer,
    User,
    Assistant,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Default)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
}

impl ChatRequest {
    pub fn with_tools(self) -> ToolingChatRequest {
        ToolingChatRequest {
            messages: self.messages,
        }
    }
}

impl ChatRequest {
    pub fn user(text: &str) -> Self {
        let message = Message {
            role: Role::User,
            content: text.to_string(),
        };
        Self {
            messages: vec![message],
        }
    }
}

impl Request for ChatRequest {
    type Response = ChatResponse;
}

#[derive(Default)]
pub struct ChatResponse {
    pub messages: Vec<Message>,
}

impl ChatResponse {
    pub fn squash(&self) -> String {
        let mut text = String::new();
        for msg in &self.messages {
            text.push_str(&msg.content);
        }
        text
    }
}

#[derive(Default)]
pub struct ToolingChatRequest {
    pub messages: Vec<Message>,
}

impl Request for ToolingChatRequest {
    type Response = ToolingChatResponse;
}

#[derive(Default)]
pub struct ToolingChatResponse {
    pub messages: Vec<Message>,
}

impl ToolingChatResponse {
    pub fn without_tools(self) -> ChatResponse {
        ChatResponse {
            messages: self.messages,
        }
    }
}
