use super::{ReasoningRouter, RouterLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, MessageFor, OnRequest};
use crb::superagent::{Fetcher, Interaction, Request, Responder};
use derive_more::{Deref, DerefMut};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::any::type_name;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait CallParameters: DeserializeOwned + Send + 'static {}

impl<T> CallParameters for T where T: DeserializeOwned + Send + 'static {}

#[async_trait]
pub trait Tool<P>
where
    Self: Agent,
    P: CallParameters,
{
    fn name(&self) -> String {
        // TODO: Use `const_str!`
        type_name::<Self>()
            .to_lowercase()
            .replace("::", "_")
            .replace('<', "_")
            .replace('>', "")
    }

    fn description(&self) -> Option<String> {
        None
    }

    async fn handle_request(
        &mut self,
        msg: Interaction<ToolRequest>,
        ctx: &mut Self::Context,
    ) -> Result<()> {
        match serde_json::from_value(msg.request.value) {
            Ok(request) => self.handle_response(request, msg.responder, ctx).await,
            Err(err) => msg.responder.send_result(Err(err.into())),
        }
    }

    async fn handle_response(
        &mut self,
        msg: P,
        responder: Responder<ToolResponse>,
        ctx: &mut Self::Context,
    ) -> Result<()> {
        let res = self.on_request(msg, ctx).await;
        responder.send_result(res)
    }

    async fn on_request(&mut self, _msg: P, _ctx: &mut Self::Context) -> Result<ToolResponse> {
        Err(anyhow!("Not implemented"))
    }
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

struct ToolRoute<P> {
    _type: PhantomData<P>,
}

unsafe impl<P> Sync for ToolRoute<P> {}

impl<A, P> ToolAddress for (Address<A>, ToolRoute<P>)
where
    A: Tool<P>,
    P: CallParameters,
{
    fn call_tool(&self, value: Value) -> Fetcher<ToolResponse> {
        let request = ToolRequest { value };
        let (interaction, fetcher) = Interaction::new_pair(request);
        let msg = CallTool {
            _type: PhantomData::<P>,
            interaction,
        };
        let res = self.0.send(msg);
        fetcher.grasp(res)
    }
}

impl RouterLink {
    pub async fn add_tool<A, P>(&mut self, addr: Address<A>, meta: ToolMeta) -> Result<ToolId>
    where
        A: Tool<P>,
        P: CallParameters,
    {
        let call = ToolRoute {
            _type: PhantomData::<P>,
        };
        let link = ToolLink {
            address: Arc::new((addr, call)),
        };
        let msg = AddTool { link, meta };
        let response = self.address.interact(msg).await?;
        Ok(response.info.id.clone())
    }
}

pub type ToolId = String;

#[derive(Debug)]
pub struct ToolMeta {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

pub struct AddTool {
    link: ToolLink,
    meta: ToolMeta,
}

pub struct ToolAdded {
    pub info: ToolInfo,
}

impl Request for AddTool {
    type Response = ToolAdded;
}

#[derive(Debug)]
pub struct ToolMetaWithId {
    pub id: ToolId,
    pub meta: ToolMeta,
}

#[derive(Deref, DerefMut, Clone, Debug)]
pub struct ToolInfo {
    pub meta: Arc<ToolMetaWithId>,
}

pub struct ToolRecord {
    pub link: ToolLink,
    pub info: ToolInfo,
}

#[async_trait]
impl OnRequest<AddTool> for ReasoningRouter {
    async fn on_request(&mut self, msg: AddTool, _ctx: &mut Self::Context) -> Result<ToolAdded> {
        let id = ToolId::from(format!("{}_{}", msg.meta.name, self.tools.len()));
        let meta = ToolMetaWithId {
            id: id.clone(),
            meta: msg.meta,
        };
        let info = ToolInfo {
            meta: Arc::new(meta),
        };
        let record = ToolRecord {
            link: msg.link,
            info: info.clone(),
        };
        self.tools.insert(id, record);
        Ok(ToolAdded { info })
    }
}

pub struct ToolRequest {
    pub value: Value,
}

impl Request for ToolRequest {
    type Response = ToolResponse;
}

struct CallTool<P> {
    _type: PhantomData<P>,
    interaction: Interaction<ToolRequest>,
}

#[async_trait]
impl<A, P> MessageFor<A> for CallTool<P>
where
    A: Tool<P>,
    P: CallParameters,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut A::Context) -> Result<()> {
        agent.handle_request(self.interaction, ctx).await
    }
}
