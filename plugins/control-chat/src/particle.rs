use crate::flow::{Chat, ChatAction};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::superagent::{StreamSession, Supervisor};
use ice9_core::{Particle, SubstanceLinks};
use ui9_dui::{Act, Pub};

pub struct ChatParticle {
    chat: Pub<Chat>,
}

impl Particle for ChatParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            chat: Pub::unified(),
        }
    }
}

impl Supervisor for ChatParticle {
    type GroupBy = ();
}

impl Agent for ChatParticle {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ChatParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let actions = self.chat.actions()?;
        ctx.consume(actions.into_events_stream());
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Chat>> for ChatParticle {
    async fn handle(&mut self, msg: Act<Chat>, _ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            ChatAction::Request { question } => {
                self.chat.start_thinking("...");
                // TODO: Send a request to a substance
            }
        }
        Ok(())
    }
}
