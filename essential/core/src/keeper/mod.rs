mod updates;

use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, AgentSession, DoSync, Next, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

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

impl Agent for Keeper {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_sync(LoadDotEnv)
    }
}

struct LoadDotEnv;

impl DoSync<LoadDotEnv> for Keeper {
    fn once(&mut self, _: &mut LoadDotEnv) -> Result<Next<Self>> {
        dotenvy::dotenv()?;
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
