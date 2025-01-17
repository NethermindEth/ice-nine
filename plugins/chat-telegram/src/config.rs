use ice_nine_core::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TelegramConfig {
    pub api_key: String,
}

impl Config for TelegramConfig {
    const NAMESPACE: &str = "telegram";

    fn template() -> Self {
        Self {
            api_key: "API KEY HERE".into(),
        }
    }
}
