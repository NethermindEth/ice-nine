use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next};
use crb::core::types::Slot;
use ice_nine_core::{Config, KeeperClient, Particle, ParticleSetup};
use serde::Deserialize;
use teloxide_core::{payloads::GetUpdatesSetters, prelude::Requester, types::UpdateKind, Bot};

pub struct TelegramParticle {
    keeper: KeeperClient,
    bot: Slot<Bot>,
    offset: i32,
}

impl Particle for TelegramParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            keeper: setup.keeper,
            bot: Slot::empty(),
            offset: 0,
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
        self.bot.fill(bot)?;
        Ok(Next::do_async(DrainUpdates))
    }
}

struct DrainUpdates;

#[async_trait]
impl DoAsync<DrainUpdates> for TelegramParticle {
    async fn repeat(&mut self, _: &mut DrainUpdates) -> Result<Option<Next<Self>>> {
        let bot = self.bot.get_mut()?;
        let updates = bot.get_updates().offset(self.offset).await?;
        for update in updates {
            self.offset = update.id.as_offset();
            if let UpdateKind::Message(message) = update.kind {
                if let Some(text) = message.text() {
                    bot.send_message(message.chat.id, text).await?;
                }
            }
        }
        Ok(None)
    }
}
