use anthropic_sdk::Client;
use ice9_core::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AnthropicConfig {
    api_key: String,
    version: String,
}

impl Config for AnthropicConfig {
    const NAMESPACE: &str = "anthropic";

    fn template() -> Self {
        Self {
            api_key: "API KEY HERE".into(),
            version: "2023-06-01".into(),
        }
    }
}

impl AnthropicConfig {
    pub fn extract(&self) -> Client {
        Client::new().auth(&self.api_key).version(&self.version)
    }
}