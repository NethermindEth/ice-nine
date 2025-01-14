use ice_nine_core::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DyDxConfig {}

impl Config for DyDxConfig {
    const NAMESPACE: &str = "dydx";
}
