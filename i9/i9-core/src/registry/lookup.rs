use super::Registry;
use anyhow::{anyhow as err, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, OnRequest, Request};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use typedmap::TypedMapKey;

pub struct Lookup<A> {
    _type: PhantomData<A>,
}

impl<A: Agent> Request for Lookup<A> {
    type Response = Address<A>;
}

#[async_trait]
impl<A: Agent> OnRequest<Lookup<A>> for Registry {
    async fn on_request(&mut self, lookup: Lookup<A>, _: &mut Self::Context) -> Result<Address<A>> {
        let address = self
            .links
            .get(&lookup)
            .ok_or_else(|| err!("Actor has not registered"))?;
        Ok(address.clone())
    }
}

unsafe impl<A> Sync for Lookup<A> {}

impl<A: Agent> TypedMapKey for Lookup<A> {
    type Value = Address<A>;
}

impl<A: Agent> PartialEq for Lookup<A> {
    fn eq(&self, other: &Self) -> bool {
        self._type.eq(&other._type)
    }
}

impl<A: Agent> Eq for Lookup<A> {}

impl<A: Agent> Hash for Lookup<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._type.hash(state);
    }
}
