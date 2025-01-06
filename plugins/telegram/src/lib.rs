use anyhow::{anyhow as err, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next};
use ice_nine_core::{Config, KeeperClient, Particle, ParticleSetup};
use serde::Deserialize;
use teloxide_core::{prelude::Requester, types::UpdateKind, Bot};

pub struct TelegramParticle {
    keeper: KeeperClient,
    bot: Option<Bot>,
}

impl Particle for TelegramParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            keeper: setup.keeper,
            bot: None,
        }
    }
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
        println!("Configuring...");
        let config: TelegramConfig = self.keeper.get_config().await?;
        let bot = Bot::new(&config.api_key);
        bot.get_me().await?;
        self.bot = Some(bot);
        Ok(Next::do_async(DrainUpdates))
    }
}

struct DrainUpdates;

#[async_trait]
impl DoAsync<DrainUpdates> for TelegramParticle {
    async fn repeat(&mut self, _: &mut DrainUpdates) -> Result<Option<Next<Self>>> {
        let bot = self
            .bot
            .as_mut()
            .ok_or_else(|| err!("Bot has not configured"))?;
        let updates = bot.get_updates().await?;
        for update in updates {
            if let UpdateKind::Message(message) = update.kind {
                if let Some(text) = message.text() {
                    bot.send_message(message.chat.id, text).await?;
                }
            }
        }
        Ok(None)
    }
}
