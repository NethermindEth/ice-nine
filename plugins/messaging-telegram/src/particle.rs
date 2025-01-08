use crate::drainer::TelegramDrainer;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Agent, Context, DoAsync, InContext, Next, OnEvent, Supervisor, SupervisorSession,
};
use crb::core::types::Slot;
use ice_nine_core::{Config, KeeperClient, Particle, ParticleSetup};
use serde::Deserialize;
use teloxide_core::{prelude::Requester, types::Message, Bot};

pub struct TelegramParticle {
    keeper: KeeperClient,
    bot: Slot<Bot>,
}

impl Supervisor for TelegramParticle {
    type GroupBy = ();
}

impl Particle for TelegramParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            keeper: setup.keeper,
            bot: Slot::empty(),
        }
    }
}

impl Agent for TelegramParticle {
    type Context = SupervisorSession<Self>;
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

        Ok(Next::in_context(SpawnWorkers))
    }
}

struct SpawnWorkers;

#[async_trait]
impl InContext<SpawnWorkers> for TelegramParticle {
    async fn handle(&mut self, _: SpawnWorkers, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let bot = self.bot.get_mut()?.clone();
        let drainer = TelegramDrainer::new(ctx.address().clone(), bot);
        ctx.spawn_agent(drainer, ());
        Ok(Next::process())
    }
}

#[async_trait]
impl OnEvent<Message> for TelegramParticle {
    type Error = Error;

    async fn handle(&mut self, message: Message, _ctx: &mut Self::Context) -> Result<()> {
        let bot = self.bot.get_mut()?;
        if let Some(text) = message.text() {
            bot.send_message(message.chat.id, text).await?;
        }
        Ok(())
    }
}
