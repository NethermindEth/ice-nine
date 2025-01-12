use super::{ReasoningRouter, RouterLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Equip, MessageFor, OnEvent};
use crb::superagent::{Fetcher, Interaction, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait Tool<IN>
where
    Self: OnRequest<IN>,
    IN: Request<Response = ToolResponse>,
{
}

pub struct ToolResponse {
    pub content: String,
}

#[derive(Deref, DerefMut, Clone)]
pub struct ToolLink {
    address: Arc<dyn ToolAddress>,
}

pub trait ToolAddress: Sync + Send {
    fn call_tool(&self, value: Value) -> Fetcher<ToolResponse>;
}

struct ToolRoute<R> {
    _type: PhantomData<R>,
}

unsafe impl<R> Sync for ToolRoute<R> {}

impl<A, R> ToolAddress for (Address<A>, ToolRoute<R>)
where
    A: Tool<R> + OnRequest<R>,
    R: Request<Response = ToolResponse> + DeserializeOwned,
{
    fn call_tool(&self, value: Value) -> Fetcher<ToolResponse> {
        let request = ToolRequest { value };
        let (interaction, fetcher) = Interaction::new_pair(request);
        let msg = CallTool {
            _type: PhantomData::<R>,
            interaction,
        };
        let res = self.0.send(msg);
        fetcher.grasp(res)
    }
}

impl RouterLink {
    pub fn add_tool<A, R>(&mut self, addr: Address<A>, meta: ToolMeta) -> Result<()>
    where
        A: Tool<R> + OnRequest<R>,
        R: Request<Response = ToolResponse> + DeserializeOwned,
    {
        let call = ToolRoute {
            _type: PhantomData::<R>,
        };
        let link = ToolLink {
            address: Arc::new((addr, call)),
        };
        let msg = AddTool { link, meta };
        self.address.event(msg)?;
        Ok(())
    }
}

pub type ToolId = String;

pub struct ToolMeta {
    name: String,
    description: Option<String>,
    parameters: Option<Value>,
}

pub struct AddTool {
    meta: ToolMeta,
    link: ToolLink,
}

#[async_trait]
impl OnEvent<AddTool> for ReasoningRouter {
    async fn handle(&mut self, msg: AddTool, _ctx: &mut Self::Context) -> Result<()> {
        let tool_id = format!("{}_{}", msg.meta.name, self.tools.len());
        self.tools.insert(tool_id, msg.link);
        Ok(())
    }
}

struct ToolRequest {
    value: Value,
}

impl Request for ToolRequest {
    type Response = ToolResponse;
}

struct CallTool<R> {
    _type: PhantomData<R>,
    interaction: Interaction<ToolRequest>,
}

#[async_trait]
impl<A, R> MessageFor<A> for CallTool<R>
where
    A: Tool<R> + OnRequest<R>,
    R: Request<Response = ToolResponse> + DeserializeOwned,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut A::Context) -> Result<()> {
        let Interaction { request, responder } = self.interaction;
        match serde_json::from_value(request.value) {
            Ok(request) => {
                let interaction = Interaction { request, responder };
                OnRequest::handle(agent, interaction, ctx).await
            }
            Err(err) => responder.send_result(Err(err.into())),
        }
    }
}
