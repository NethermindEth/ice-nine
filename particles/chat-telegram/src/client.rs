use anyhow::Result;
use derive_more::{Deref, DerefMut};
use teloxide_core::{
    prelude::Requester,
    types::{ChatAction, ChatId},
    Bot,
};

#[derive(Deref, DerefMut, Clone)]
pub struct Client {
    bot: Bot,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        let bot = Bot::new(api_key);
        Self { bot }
    }

    pub async fn typing(&mut self, chat_id: ChatId) -> Result<()> {
        self.bot
            .send_chat_action(chat_id, ChatAction::Typing)
            .await?;
        Ok(())
    }
}
