use super::ReasoningRouter;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, Equip, OnEvent};
use crb::superagent::Fetcher;
use derive_more::{Deref, DerefMut, From};
use std::sync::Arc;

pub trait Tool: Agent {}

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

pub trait ToolAddress: Sync + Send {}

impl<T: Tool> ToolAddress for Address<T> {}

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<ReasoningRouter>,
}

impl RouterLink {
    pub fn add_tool<T>(&mut self, addr: Address<T>) -> Result<()>
    where
        T: Tool,
    {
        let msg = AddTool { link: addr.equip() };
        self.address.event(msg)?;
        Ok(())
    }
}

pub struct AddTool {
    link: ToolLink,
}

#[async_trait]
impl OnEvent<AddTool> for ReasoningRouter {
    type Error = Error;

    async fn handle(&mut self, msg: AddTool, _ctx: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}
