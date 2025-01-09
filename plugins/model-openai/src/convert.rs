use async_openai::types::*;
use ice_nine_core::{Message as ModelMessage, Role as ModelRole};

pub fn message(from: ModelMessage) -> ChatCompletionRequestMessage {
    match from.role {
        ModelRole::Developer => {
            let mut message = ChatCompletionRequestSystemMessage::default();
            let content = ChatCompletionRequestSystemMessageContent::Text(from.content);
            message.content = content;
            ChatCompletionRequestMessage::from(message)
        }
        ModelRole::User => {
            let mut message = ChatCompletionRequestUserMessage::default();
            let content = ChatCompletionRequestUserMessageContent::Text(from.content);
            message.content = content;
            ChatCompletionRequestMessage::from(message)
        }
    }
}

pub fn choice(from: ChatChoice) -> Option<ModelMessage> {
    let role = match from.message.role {
        Role::System => ModelRole::Developer,
        Role::User => ModelRole::User,
        _ => {
            return None;
        }
    };
    let content = from.message.content?;
    let message = ModelMessage { role, content };
    Some(message)
}
