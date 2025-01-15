use super::{Config, Keeper, KeeperLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::Context;
use crb::superagent::{InteractExt, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use std::marker::PhantomData;
use toml::Value;

impl KeeperLink {
    pub async fn get_config<C>(&self) -> Result<C>
    where
        C: Config,
    {
        let request = GetConfig::<C> {
            namespace: C::NAMESPACE.to_string(),
            _type: PhantomData,
        };
        let config = self.address.interact(request).await?;
        Ok(config)
    }
}

pub struct GetConfig<C> {
    namespace: String,
    _type: PhantomData<C>,
}

impl<C: Config> Request for GetConfig<C> {
    type Response = C;
}

#[async_trait]
impl<C: Config> OnRequest<GetConfig<C>> for Keeper {
    async fn on_request(&mut self, msg: GetConfig<C>, _: &mut Context<Self>) -> Result<C> {
        let mut ns = &msg.namespace;
        let value = self
            .config
            .as_ref()
            .ok_or_else(|| anyhow!("Config has not loaded yet"))?;
        let config = get_config(value, ns)
            .ok_or_else(|| anyhow!("Can't parse the config"))?
            .try_into()?;
        Ok(config)
    }
}

// TODO: Return error instead and use that in logs
fn get_config(value: &Value, namespace: &str) -> Option<Value> {
    value
        .get("particle")?
        .get(namespace)?
        .get("config")
        .cloned()
}
