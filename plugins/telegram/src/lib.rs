use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next};
use ice_nine_core::{Config, KeeperAddress};
use serde::Deserialize;

pub struct TelegramParticle {
    keeper: KeeperAddress,
}

impl Agent for TelegramParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Configure)
    }
}

#[derive(Deserialize)]
struct TelegramConfig {
    api_key: String,
}

impl Config for TelegramConfig {
    const NAMESPACE: &'static str = "TELEGRAM";
}

struct Configure;

#[async_trait]
impl DoAsync<Configure> for TelegramParticle {
    async fn once(&mut self, _: &mut Configure) -> Result<Next<Self>> {
        let config: TelegramConfig = self.keeper.get_config().await?;
        println!("Config for telegram is loaded!");
        Ok(Next::todo("Not yet implemented!"))
    }
}
