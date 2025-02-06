use anthropic_sdk::Client;
use dotenvy::dotenv;
use ice9_core::Config;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct AnthropicConfig {
    pub version: String,
    pub model: String,
    pub max_tokens: i32,
}

impl Config for AnthropicConfig {
    const NAMESPACE: &str = "anthropic";

    fn template() -> Self {
        Self {
            version: "2023-06-01".into(),
            model: "claude-3-opus-20240229".into(),
            max_tokens: 1024,
        }
    }
}

impl AnthropicConfig {
    pub fn extract(&self) -> Result<Client, env::VarError> {
        dotenv().ok();
        let api_key = env::var("ANTHROPIC_API_KEY")?;

        let client = Client::new()
            .auth(api_key.as_str())
            .version(self.version.as_str());
        Ok(client)
    }
}
