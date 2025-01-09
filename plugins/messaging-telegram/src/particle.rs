use crate::config::TelegramConfig;
use crate::drainer::TelegramDrainer;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Agent, Context, DoAsync, InContext, Next, OnEvent, Supervisor, SupervisorSession,
};
use crb::core::types::Slot;
use ice_nine_core::{ChatRequest, ModelLink, Particle, ParticleSetup, SubstanceLinks};
use std::collections::HashSet;
use teloxide_core::{
    prelude::Requester,
    types::{ChatId, Message},
    Bot,
};

const NAMESPACE: &'static str = "TELEGRAM";

pub struct TelegramParticle {
    links: SubstanceLinks,
    model: ModelLink,
    client: Slot<Bot>,

    typing: HashSet<ChatId>,
}

impl Supervisor for TelegramParticle {
    type GroupBy = ();
}

impl Particle for TelegramParticle {
    fn construct(setup: ParticleSetup) -> Self {
        let model = setup.links.router.model();
        Self {
            links: setup.links,
            model,
            client: Slot::empty(),
            typing: HashSet::new(),
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

struct Configure;

#[async_trait]
impl DoAsync<Configure> for TelegramParticle {
    async fn once(&mut self, _: &mut Configure) -> Result<Next<Self>> {
        println!("Configuring...");
        let config: TelegramConfig = self.links.keeper.get_config(NAMESPACE).await?;
        let client = Bot::new(&config.api_key);
        client.get_me().await?;
        self.client.fill(client)?;
        Ok(Next::in_context(SpawnWorkers))
    }
}

struct SpawnWorkers;

#[async_trait]
impl InContext<SpawnWorkers> for TelegramParticle {
    async fn handle(&mut self, _: SpawnWorkers, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let client = self.client.get_mut()?.clone();
        let address = ctx.address().clone();
        let drainer = TelegramDrainer::new(address, client);
        ctx.spawn_agent(drainer, ());
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Message> for TelegramParticle {
    type Error = Error;

    async fn handle(&mut self, message: Message, _ctx: &mut Self::Context) -> Result<()> {
        let client = self.client.get_mut()?;
        if let Some(text) = message.text() {
            let chat_id = message.sender_chat.as_ref().map(|chat| chat.id);
            if let Some(chat_id) = chat_id {
                self.typing.insert(chat_id);
            }

            let request = ChatRequest::user(&text);
            let response = self.model.chat(request).await?;
            let text = response.squash();
            client.send_message(message.chat.id, text).await?;
        }
        Ok(())
    }
}

/*
    async move {
        loop {
            bot.send_chat_action(chat_id, ChatAction::Typing).await?;
            typing_interval.tick().await;
        }
    }
*/
