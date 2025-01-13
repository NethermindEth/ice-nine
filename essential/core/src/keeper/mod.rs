mod updates;

use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, Context, Duty, Next, OnEvent, OnRequest, Request};
use crb::agent::{Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use ice_nine_std::config_loader::ConfigLoader;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use toml::Value;

pub trait Config: DeserializeOwned + Send + 'static {
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

pub struct Keeper {}

impl Keeper {
    pub fn new() -> Self {
        Self {}
    }
}

impl Supervisor for Keeper {
    type GroupBy = ();
}

impl Agent for Keeper {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(LoadDotEnv)
    }
}

struct LoadDotEnv;

#[async_trait]
impl Duty<LoadDotEnv> for Keeper {
    async fn handle(&mut self, _: LoadDotEnv, ctx: &mut Self::Context) -> Result<Next<Self>> {
        dotenvy::dotenv()?;
        let recipient = ctx.address().recipient();
        let loader = ConfigLoader::new(recipient);
        ctx.spawn_agent(loader, ());
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
        let mut ns = msg.namespace.to_uppercase();
        ns.push('_');
        let config: C = envy::prefixed(ns).from_env()?;
        Ok(config)
    }
}

#[async_trait]
impl OnEvent<Value> for Keeper {
    async fn handle(&mut self, value: Value, ctx: &mut Self::Context) -> Result<()> {
        println!("Config updated: {:?}", value);
        // TODO: Send updates
        Ok(())
    }
}
