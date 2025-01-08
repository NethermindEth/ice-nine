use serde::Deserialize;

#[derive(Deserialize)]
pub struct TelegramConfig {
    pub api_key: String,
}
