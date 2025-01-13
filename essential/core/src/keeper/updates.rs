use super::{Config, Keeper, KeeperLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, MessageFor};
use crb::send::MessageSender;
use crb::superagent::{OnRequest, Request};

#[async_trait]
pub trait UpdateConfig<C: Config>: Agent {
    async fn update_config(&mut self, config: C, ctx: &mut Self::Context) -> Result<()>;
}

impl KeeperLink {
    /// Subscribe to live configuration updates.
    pub async fn subscribe<A, C>(&self, address: Address<A>) -> Result<()>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let msg = Subscribe::<C> {
            sender: address.sender(),
        };
        self.interact(msg).await?;
        Ok(())
    }
}

pub struct UpdateConfigEvent<C> {
    config: C,
}

#[async_trait]
impl<A, C> MessageFor<A> for UpdateConfigEvent<C>
where
    A: UpdateConfig<C>,
    C: Config,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut A::Context) -> Result<()> {
        agent.update_config(self.config, ctx).await
    }
}

pub struct Subscribe<C> {
    sender: MessageSender<UpdateConfigEvent<C>>,
}

impl<C: Config> Request for Subscribe<C> {
    type Response = ();
}

#[async_trait]
impl<C: Config> OnRequest<Subscribe<C>> for Keeper {
    async fn on_request(&mut self, msg: Subscribe<C>, _: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}
