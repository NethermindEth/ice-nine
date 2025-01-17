use ice_nine_core::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StdioConfig {
    pub loader: String,
}

impl Default for StdioConfig {
    fn default() -> Self {
        Self { loader: ".".into() }
    }
}

impl Config for StdioConfig {
    const NAMESPACE: &str = "stdio";

    fn template() -> Self {
        Self::default()
    }
}
