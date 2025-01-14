pub mod updates;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next};
use crb::superagent::{AddressExt, OnRequest, Request, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use ice_nine_std::config_loader::ConfigLoader;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use toml::Value;
use updates::ConfigUpdater;

pub trait Config: DeserializeOwned + Send + 'static {
    // TODO: Add scope
    // TODO: Add methods to get a full path for logging
    const NAMESPACE: &str;
}

#[derive(Deref, DerefMut, From, Clone)]
pub struct KeeperLink {
    address: Address<Keeper>,
}

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

pub struct Keeper {
    config: Option<Value>,
    listeners: Vec<ConfigUpdater>,
}

impl Keeper {
    pub fn new() -> Self {
        Self {
            config: None,
            listeners: Vec::new(),
        }
    }
}

impl Supervisor for Keeper {
    type GroupBy = ();
}

impl Agent for Keeper {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(SpawnWatcher)
    }
}

struct SpawnWatcher;

#[async_trait]
impl Duty<SpawnWatcher> for Keeper {
    async fn handle(&mut self, _: SpawnWatcher, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let recipient = ctx.address().recipient();
        let loader = ConfigLoader::new(recipient);
        let addr = ctx.spawn_agent(loader, ());
        // TODO: Use `addr` interaction to load the initial config
        // TODO: Implement a subscribe method for loader that will
        // return a current configuration, than updates
        Ok(Next::events())
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
    async fn on_request(&mut self, msg: GetConfig<C>, _: &mut Self::Context) -> Result<C> {
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
