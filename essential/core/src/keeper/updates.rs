use super::{Config, Keeper, KeeperLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, MessageFor};
use crb::send::{Recipient, Sender};
use crb::superagent::{OnRequest, Request};
use std::marker::PhantomData;
use std::sync::Arc;
use toml::Value;

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

pub struct ConfigListener {
    recipient: Recipient<Arc<Value>>,
}

impl ConfigListener {
    pub fn send_new_config(&self, value: Arc<Value>) -> Result<()> {
        self.recipient.send(value)
    }
}

pub struct ConfigListenerAddress<C: Config> {
    recipient: Recipient<UpdateConfigEvent<C>>,
}

impl<C> Sender<Value> for ConfigListenerAddress<C>
where
    C: Config,
{
    fn send(&self, value: Value) -> Result<()> {
        let event = UpdateConfigEvent {
            _type: PhantomData::<C>,
            value,
        };
        self.recipient.send(event)?;
        Ok(())
    }
}

pub struct UpdateConfigEvent<C> {
    _type: PhantomData<C>,
    value: Value,
}

#[async_trait]
impl<A, C> MessageFor<A> for UpdateConfigEvent<C>
where
    A: UpdateConfig<C>,
    C: Config,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut A::Context) -> Result<()> {
        let config: C = self.value.try_into()?;
        agent.update_config(config, ctx).await
    }
}

pub struct Subscribe<C> {
    sender: Recipient<UpdateConfigEvent<C>>,
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
