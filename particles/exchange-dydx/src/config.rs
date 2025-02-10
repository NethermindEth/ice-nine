use n9_core::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DyDxConfig {}

impl Config for DyDxConfig {
    const NAMESPACE: &str = "dydx";

    fn template() -> Self {
        Self {}
    }
}
