use n9_core::{Message as ModelMessage, Role as ModelRole};

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnthropicRole {
    User,
    Assistant,
    System,
}

#[derive(serde::Serialize)]
pub struct AnthropicMessage {
    pub role: AnthropicRole,
    pub content: String,
}

pub fn message(from: ModelMessage) -> AnthropicMessage {
    let role = match from.role {
        ModelRole::User => AnthropicRole::User,
        ModelRole::Assistant => AnthropicRole::Assistant,
        ModelRole::Developer => AnthropicRole::System,
    };

    AnthropicMessage {
        role,
        content: from.content,
    }
}

pub fn choice(from: &serde_json::Value) -> Option<ModelMessage> {
    let role_str = from.get("role")?.as_str()?;
    let role = match role_str {
        "user" => ModelRole::User,
        "assistant" => ModelRole::Assistant,
        "system" => ModelRole::Developer,
        _ => return None,
    };

    let content = from
        .get("content")
        .and_then(serde_json::Value::as_str)?
        .to_string();

    let message = ModelMessage { role, content };
    Some(message)
}
