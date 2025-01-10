use async_openai::{config::OpenAIConfig as RawConfig, Client as OpenAIClient};
use ice_nine_core::Config;
use serde::Deserialize;

pub type Client = OpenAIClient<RawConfig>;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct OpenAIConfig(pub RawConfig);

impl Config for OpenAIConfig {
    const NAMESPACE: &str = "OPENAI";
}
