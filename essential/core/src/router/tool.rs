use super::{ReasoningRouter, RouterLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Equip, MessageFor, OnEvent};
use crb::superagent::{AddressExt, Fetcher, Interaction, OnRequest, Request, Responder};
use derive_more::{Deref, DerefMut, From};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;

pub trait Tool: OnRequest<Self::Request> {
    type Request: Request<Response = ToolResponse> + DeserializeOwned;
}

pub struct ToolResponse {
    pub content: String,
}

#[derive(Deref, DerefMut, Clone)]
pub struct ToolLink {
    address: Arc<dyn ToolAddress>,
}

impl<T: Tool> From<Address<T>> for ToolLink {
    fn from(addr: Address<T>) -> Self {
        Self {
            address: Arc::new(addr),
        }
    }
}

pub trait ToolAddress: Sync + Send {
    fn call_tool(&self, value: Value) -> Fetcher<ToolResponse>;
}

impl<T: Tool> ToolAddress for Address<T> {
    fn call_tool(&self, value: Value) -> Fetcher<ToolResponse> {
        let request = ToolRequest { value };
        let (interaction, fetcher) = Interaction::new_pair(request);
        let msg = CallTool { interaction };
        let res = self.send(msg);
        fetcher.grasp(res)
    }
}

impl RouterLink {
    pub fn add_tool<T>(&mut self, addr: Address<T>, meta: ToolMeta) -> Result<()>
    where
        T: Tool,
    {
        let msg = AddTool {
            link: addr.equip(),
            meta,
        };
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

struct CallTool {
    interaction: Interaction<ToolRequest>,
}

#[async_trait]
impl<T> MessageFor<T> for CallTool
where
    T: Tool,
{
    async fn handle(self: Box<Self>, agent: &mut T, ctx: &mut T::Context) -> Result<()> {
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
