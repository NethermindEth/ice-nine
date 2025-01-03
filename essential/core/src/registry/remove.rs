use crate::registry::{lookup::Lookup, Registry};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, OnEvent};
use crb::send::{DropNotifier, Sender};
use std::marker::PhantomData;

pub struct Remove<A> {
    _type: PhantomData<A>,
}

unsafe impl<A> Sync for Remove<A> {}
unsafe impl<A> Send for Remove<A> {}

impl Registry {
    pub fn remover_for<A>(&self, ctx: &mut <Self as Agent>::Context) -> DropNotifier
    where
        A: Agent,
    {
        let event = Remove::<A> { _type: PhantomData };
        ctx.address()
            .event_sender()
            .notifier(event)
            .once()
            .into_drop_notifier()
    }
}

#[async_trait]
impl<A> OnEvent<Remove<A>> for Registry
where
    A: Agent,
{
    type Error = Error;

    async fn handle(&mut self, _remove: Remove<A>, _ctx: &mut Self::Context) -> Result<()> {
        let key = Lookup::<A>::key();
        self.links
            .remove(&key)
            .map(drop)
            .ok_or_else(|| Error::msg("Agent has unregistered already"))
    }
}
