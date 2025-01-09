use crate::config::TelegramConfig;
use crate::drainer::TelegramDrainer;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Agent, Context, DoAsync, InContext, Next, OnEvent, OnResponse, Output, Supervisor,
    SupervisorSession,
};
use crb::core::{time::Duration, types::Slot};
use crb::superagent::{Interval, OnTick};
use ice_nine_core::{
    ChatRequest, ChatResponse, ModelLink, Particle, ParticleSetup, SubstanceLinks,
};
use std::collections::HashSet;
use teloxide_core::{
    prelude::Requester,
    types::{ChatAction, ChatId, Message},
    Bot,
};

const NAMESPACE: &'static str = "TELEGRAM";

pub struct TelegramParticle {
    links: SubstanceLinks,
    model: ModelLink,
    client: Slot<Bot>,

    typing: HashSet<ChatId>,
    interval: Option<Interval>,
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
            interval: None,
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
            let chat_id = message.chat.id;
            self.typing.insert(chat_id);
            client.send_chat_action(chat_id, ChatAction::Typing).await?;

            let request = ChatRequest::user(&text);
            let address = ctx.address().clone();
            self.model.chat(request).forward_to(address, chat_id);
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
        // TODO: Show error to the chat?
        let client = self.client.get_mut()?;
        let text = response?.squash();
        client.send_message(chat_id, text).await?;
        Ok(())
    }
}

#[async_trait]
impl OnTick for TelegramParticle {
    async fn on_tick(&mut self, _: &(), _ctx: &mut Self::Context) -> Result<()> {
        let client = self.client.get_mut()?;
        for chat_id in &self.typing {
            client
                .send_chat_action(*chat_id, ChatAction::Typing)
                .await?;
        }
        Ok(())
    }
}
