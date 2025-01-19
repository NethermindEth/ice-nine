use ice9_core::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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
