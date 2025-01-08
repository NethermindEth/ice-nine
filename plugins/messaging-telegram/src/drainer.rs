use crate::particle::TelegramParticle;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, DoAsync, Next};
use teloxide_core::{payloads::GetUpdatesSetters, prelude::Requester, types::UpdateKind, Bot};

pub struct TelegramDrainer {
    particle: Address<TelegramParticle>,
    bot: Bot,
    offset: i32,
}

impl TelegramDrainer {
    pub fn new(particle: Address<TelegramParticle>, bot: Bot) -> Self {
        Self {
            particle,
            bot,
            offset: 0,
        }
    }
}

impl Agent for TelegramDrainer {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(DrainMessages)
    }
}

struct DrainMessages;

#[async_trait]
impl DoAsync<DrainMessages> for TelegramDrainer {
    async fn repeat(&mut self, _: &mut DrainMessages) -> Result<Option<Next<Self>>> {
        let updates = self.bot.get_updates().offset(self.offset).await?;
        for update in updates {
            self.offset = update.id.as_offset();
            if let UpdateKind::Message(message) = update.kind {
                self.particle.event(message)?;
            }
        }
        Ok(None)
    }
}
