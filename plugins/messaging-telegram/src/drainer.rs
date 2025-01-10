use crate::client::Client;
use crate::particle::TelegramParticle;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, DoAsync, Next};
use teloxide_core::{payloads::GetUpdatesSetters, prelude::Requester, types::UpdateKind};

pub struct TelegramDrainer {
    particle: Address<TelegramParticle>,
    client: Client,
    offset: i32,
}

impl TelegramDrainer {
    pub fn new(particle: Address<TelegramParticle>, client: Client) -> Self {
        Self {
            particle,
            client,
            offset: 0,
        }
    }
}

impl Agent for TelegramDrainer {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(())
    }
}

#[async_trait]
impl DoAsync for TelegramDrainer {
    async fn repeat(&mut self, _: &mut ()) -> Result<Option<Next<Self>>> {
        let updates = self.client.get_updates().offset(self.offset).await?;
        for update in updates {
            self.offset = update.id.as_offset();
            if let UpdateKind::Message(message) = update.kind {
                self.particle.event(message)?;
            }
        }
        Ok(None)
    }
}
