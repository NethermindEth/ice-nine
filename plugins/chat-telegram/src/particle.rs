use crate::client::Client;
use crate::config::TelegramConfig;
use crate::drainer::TelegramDrainer;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next, OnEvent};
use crb::core::{time::Duration, Slot};
use crb::superagent::{Entry, IntervalSwitch, OnResponse, Output, Supervisor, SupervisorSession};
use ice_nine_core::{
    ChatRequest, ChatResponse, ConfigSegmentUpdates, Particle, SubstanceBond, SubstanceLinks,
    UpdateConfig,
};
use std::collections::HashSet;
use teloxide_core::{
    prelude::Requester,
    types::{ChatId, Message},
};

pub struct TelegramParticle {
    substance: SubstanceLinks,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,

    client: Slot<Client>,

    typing: HashSet<ChatId>,
    thinking_interval: IntervalSwitch<Tick>,
}

impl Supervisor for TelegramParticle {
    type GroupBy = ();
}

impl Particle for TelegramParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        let duration = Duration::from_secs(1);
        let thinking_interval = IntervalSwitch::new(duration, Tick);
        Self {
            substance,
            config_updates: None,
            bond: Slot::empty(),
            client: Slot::empty(),
            typing: HashSet::new(),
            thinking_interval,
        }
    }
}

impl Agent for TelegramParticle {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for TelegramParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&*ctx);
        let (config, entry) = bond.live_config_updates().await?;
        self.config_updates = Some(entry);
        self.update_config(config, ctx).await?;
        self.bond.fill(bond)?;

        self.thinking_interval.add_listener(ctx);
        self.thinking_interval.on();

        Ok(Next::events())
    }
}

#[async_trait]
impl UpdateConfig<TelegramConfig> for TelegramParticle {
    async fn update_config(
        &mut self,
        config: TelegramConfig,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        if self.client.is_filled() {
            self.client.take()?;
            ctx.tracker.terminate_group(());
        }

        let client = Client::new(&config.api_key);
        client.get_me().await?;
        self.client.fill(client)?;

        let client = self.client.cloned()?;
        let address = ctx.address().clone();
        let drainer = TelegramDrainer::new(address, client);
        ctx.spawn_agent(drainer, ());

        Ok(())
    }
}

#[async_trait]
impl OnEvent<Message> for TelegramParticle {
    async fn handle(&mut self, message: Message, ctx: &mut Context<Self>) -> Result<()> {
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
                .forwardable()
                .forward_to(address, chat_id);
        }
        Ok(())
    }
}

#[async_trait]
impl OnResponse<ChatResponse, ChatId> for TelegramParticle {
    async fn on_response(
        &mut self,
        response: Output<ChatResponse>,
        chat_id: ChatId,
        _ctx: &mut Context<Self>,
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

#[derive(Clone)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for TelegramParticle {
    async fn handle(&mut self, _: Tick, _ctx: &mut Context<Self>) -> Result<()> {
        if self.client.is_filled() {
            let client = self.client.get_mut()?;
            for chat_id in &self.typing {
                client.typing(*chat_id).await.ok();
            }
        }
        Ok(())
    }
}
