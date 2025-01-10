use ice_nine_core::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TelegramConfig {
    pub api_key: String,
}

impl Config for TelegramConfig {
    const NAMESPACE: &str = "TELEGRAM";
}
