use super::tool::ToolLink;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, MessageFor};
use crb::superagent::Responder;
use serde::de::DeserializeOwned;
use serde_json::Value;

/*
impl ToolLink {
    pub fn call_tool(&mut self, parameters: Value) {
    }
}

pub trait ToolRequest: DeserializeOwned + Send + 'static {
    type ToolResponse: Send + 'static;
}

pub struct RawToolInteraction<R: ToolRequest> {
    pub request: Value,
    pub responder: Responder<R::ToolResponse>,
}

#[async_trait]
impl<A, R> MessageFor<A> for RawToolInteraction<R>
where
    A: OnToolRequest<R>,
    R: ToolRequest,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut A::Context) -> Result<()> {
        let res = serde_json::from_value(self.request);
        match res {
            Ok(request) => {
                let interaction = ToolInteraction {
                    request,
                    responder: self.responder,
                };
                agent.handle(interaction, ctx).await
            }
            Err(err) => self.responder.send_result(Err(err.into())),
        }
    }
}

pub struct ToolInteraction<R: ToolRequest> {
    pub request: R,
    pub responder: Responder<R::ToolResponse>,
}

#[async_trait]
pub trait OnToolRequest<R: ToolRequest>: Agent {
    async fn handle(&mut self, msg: ToolInteraction<R>, ctx: &mut Self::Context) -> Result<()> {
        let resp = self.on_request(msg.request, ctx).await;
        msg.responder.send_result(resp)
    }

    async fn on_request(
        &mut self,
        _request: R,
        _ctx: &mut Self::Context,
    ) -> Result<R::ToolResponse> {
        Err(anyhow!("The on_request method in not implemented."))
    }
}
*/
