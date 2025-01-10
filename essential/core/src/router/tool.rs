use super::{ReasoningRouter, RouterLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Equip, OnEvent};
use crb::superagent::Fetcher;
use crb::superagent::{AddressExt, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;

pub trait Tool: OnRequest<Self::Request> {
    type Request: Request + DeserializeOwned;
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
    fn call_tool(&self, value: Value) -> Fetcher<Value>;
}

impl<T: Tool> ToolAddress for Address<T> {
    fn call_tool(&self, value: Value) -> Fetcher<Value> {
        let input: T::Request = serde_json::from_value(value).unwrap();
        // TODO: Allow to reform the fetcher
        self.interact(input);
        todo!()
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
