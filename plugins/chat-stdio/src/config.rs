use ice_nine_core::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StdioConfig {
    loader: String,
}

impl Config for StdioConfig {
    const NAMESPACE: &str = "stdio";
}
