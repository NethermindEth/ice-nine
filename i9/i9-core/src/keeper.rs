use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, OnRequest, Request};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub trait Config: DeserializeOwned + Send + 'static {
    fn namespace() -> &'static str;
}

pub struct Keeper {}

impl Agent for Keeper {
    type Context = AgentSession<Self>;
    type Output = ();
}

pub struct GetConfig<C> {
    _type: PhantomData<C>,
}

impl<C: Config> Request for GetConfig<C> {
    type Response = C;
}

#[async_trait]
impl<C: Config> OnRequest<GetConfig<C>> for Keeper {
    async fn on_request(&mut self, _: GetConfig<C>, _: &mut Self::Context) -> Result<C> {
        let mut ns = C::namespace().to_uppercase();
        ns.push('_');
        let config: C = envy::prefixed(ns).from_env()?;
        Ok(config)
    }
}
