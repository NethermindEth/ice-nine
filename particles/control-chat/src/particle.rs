use crate::flow::{Chat, ChatAction};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::superagent::{StreamSession, Supervisor};
use ice9_core::{ChatRequest, Particle, SubstanceLinks};
use ui9_dui::{Act, Pub};

pub struct ChatParticle {
    substance: SubstanceLinks,
    chat: Pub<Chat>,
}

impl Particle for ChatParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
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
        ctx.consume(actions);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Chat>> for ChatParticle {
    async fn handle(&mut self, msg: Act<Chat>, ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            ChatAction::Request { question } => {
                let ask = Ask { question };
                ctx.do_next(Next::do_async(ask));
            }
        }
        Ok(())
    }
}

struct Ask {
    question: String,
}

#[async_trait]
impl DoAsync<Ask> for ChatParticle {
    async fn handle(&mut self, msg: Ask, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.chat.thinking(true);
        let request = ChatRequest::user(&msg.question);
        let req = self.substance.router.chat(request);
        self.chat.add(msg.question);
        let resp = req.await?.squash();
        self.chat.add(resp);
        self.chat.thinking(false);
        Ok(Next::events())
    }
}
