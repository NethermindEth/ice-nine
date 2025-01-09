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
