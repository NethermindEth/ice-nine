use crate::registry::{lookup::Lookup, Registry};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, OnRequest, Request};
use crb::send::DropNotifier;

pub struct Add<A: Agent> {
    address: Address<A>,
}

impl<A: Agent> Request for Add<A> {
    type Response = Link;
}

#[async_trait]
impl<A: Agent> OnRequest<Add<A>> for Registry {
    async fn on_request(&mut self, add: Add<A>, ctx: &mut Self::Context) -> Result<Link> {
        let key = Lookup::<A>::key();
        self.links.insert(key, add.address);
        let remover = self.remover_for::<A>(ctx);
        Ok(Link { _remover: remover })
    }
}

pub struct Link {
    _remover: DropNotifier,
}
