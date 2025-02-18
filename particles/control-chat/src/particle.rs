use crate::flow::{Chat, ChatAction, Role};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::superagent::{Fetcher, StreamSession, Supervisor};
use n9_core::{ChatRequest, ChatResponse, Particle, SubstanceLinks};
use ui9_dui::{Act, Operation, Pub};

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
    type BasedOn = AgentSession<Self>;
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
        ctx.consume(self.chat.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Chat>> for ChatParticle {
    async fn handle(&mut self, msg: Act<Chat>, ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            ChatAction::Request { question } => {
                let ask = SendRequest { question };
                ctx.do_next(Next::do_async(ask));
            }
        }
        Ok(())
    }
}

struct SendRequest {
    question: String,
}

#[async_trait]
impl DoAsync<SendRequest> for ChatParticle {
    async fn handle(&mut self, msg: SendRequest, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let op = Operation::start("Sending a prompt");
        self.chat.thinking(true);
        let request = ChatRequest::user(&msg.question);
        let session = self.substance.router.new_session().await?;
        let req = session.chat(request);
        self.chat.add(msg.question, Role::Request);
        op.end("Prompt sent");
        let state = WaitResponse { req };
        Ok(Next::do_async(state))
    }
}

struct WaitResponse {
    req: Fetcher<ChatResponse>,
}

#[async_trait]
impl DoAsync<WaitResponse> for ChatParticle {
    async fn handle(&mut self, msg: WaitResponse, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let op = Operation::start("Waiting for the response");
        let resp = msg.req.await?.squash();
        self.chat.add(resp, Role::Response);
        self.chat.thinking(false);
        op.end("Response received");
        Ok(Next::events())
    }
}
