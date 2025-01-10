use crate::client::Client;
use crate::config::TelegramConfig;
use crate::drainer::TelegramDrainer;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Agent, Context, InContext, Next, OnEvent, OnResponse, Output, Supervisor, SupervisorSession,
};
use crb::core::{time::Duration, types::Slot};
use crb::superagent::{Interval, OnTick};
use ice_nine_core::{ChatRequest, ChatResponse, Particle, ParticleSetup, SubstanceLinks};
use std::collections::HashSet;
use teloxide_core::{
    prelude::Requester,
    types::{ChatId, Message},
};

pub struct TelegramParticle {
    substance: SubstanceLinks,
    client: Slot<Client>,

    typing: HashSet<ChatId>,
    interval: Option<Interval>,
}

impl Supervisor for TelegramParticle {
    type GroupBy = ();
}

impl Particle for TelegramParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            substance: setup.links,
            client: Slot::empty(),
            typing: HashSet::new(),
            interval: None,
        }
    }
}

impl Agent for TelegramParticle {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::in_context(Configure)
    }
}

struct Configure;

#[async_trait]
impl InContext<Configure> for TelegramParticle {
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        println!("Configuring...");
        let config: TelegramConfig = self.substance.config().await?;
        let client = Client::new(&config.api_key);
        client.get_me().await?;
        self.client.fill(client)?;

        let client = self.client.cloned()?;
        let address = ctx.address().clone();
        let drainer = TelegramDrainer::new(address, client);
        ctx.spawn_agent(drainer, ());

        let address = ctx.address().clone();
        let duration = Duration::from_secs(1);
        let interval = Interval::new(address, duration, ());
        self.interval = Some(interval);

        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Message> for TelegramParticle {
    type Error = Error;

    async fn handle(&mut self, message: Message, ctx: &mut Self::Context) -> Result<()> {
        let client = self.client.get_mut()?;
        if let Some(text) = message.text() {
            if text.starts_with('/') {
                // TODO: Commands handling
                return Ok(());
            }
            let chat_id = message.chat.id;
            self.typing.insert(chat_id);
            client.typing(chat_id).await.ok();

            let request = ChatRequest::user(&text);
            let address = ctx.address().clone();
            self.substance
                .router
                .chat(request)
                .forward_to(address, chat_id);
        }
        Ok(())
    }
}

#[async_trait]
impl OnResponse<ChatRequest, ChatId> for TelegramParticle {
    async fn on_response(
        &mut self,
        response: Output<ChatResponse>,
        chat_id: ChatId,
        _ctx: &mut Self::Context,
    ) -> Result<()> {
        self.typing.remove(&chat_id);
        let client = self.client.get_mut()?;
        // TODO: Show error to the chat?
        let text = response?.squash();
        client.send_message(chat_id, text).await?;
        // The message sending cleans a typing status
        Ok(())
    }
}

#[async_trait]
impl OnTick for TelegramParticle {
    async fn on_tick(&mut self, _: &(), _ctx: &mut Self::Context) -> Result<()> {
        let client = self.client.get_mut()?;
        for chat_id in &self.typing {
            client.typing(*chat_id).await.ok();
        }
        Ok(())
    }
}
