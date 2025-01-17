use async_openai::{config::OpenAIConfig as RawConfig, Client as OpenAIClient};
use ice_nine_core::Config;
use serde::{Deserialize, Serialize};

pub type Client = OpenAIClient<RawConfig>;

#[derive(Deserialize, Serialize)]
pub struct OpenAIConfig {
    api_key: String,
}

impl Config for OpenAIConfig {
    const NAMESPACE: &str = "openai";

    fn template() -> Self {
        Self {
            api_key: "API KEY HERE".into(),
        }
    }
}

impl OpenAIConfig {
    pub fn extract(&self) -> RawConfig {
        RawConfig::default().with_api_key(&self.api_key)
    }
}
