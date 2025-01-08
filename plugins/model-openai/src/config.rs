use ice_nine_core::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
}

impl Config for OpenAIConfig {
    const NAMESPACE: &'static str = "OPENAI";
}
