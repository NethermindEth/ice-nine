use super::{Config, Keeper, KeeperLink};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, MessageFor};
use crb::send::{Recipient, Sender};
use crb::superagent::{OnRequest, Request};
use std::any::type_name;
use std::marker::PhantomData;
use toml::Value;

#[async_trait]
pub trait UpdateConfig<C: Config>: Agent {
    async fn update_config(&mut self, config: C, ctx: &mut Self::Context) -> Result<()>;

    fn fallback(&mut self, err: Error, ctx: &mut Self::Context) {
        log::error!("Can't load the config {}: {err}", type_name::<C>());
    }
}

impl KeeperLink {
    /// Subscribe to live configuration updates.
    pub async fn subscribe<A, C>(&self, address: Address<A>) -> Result<()>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let recipient = TypedConfigListener {
            recipient: address.sender(),
        };
        let updater = ConfigUpdater {
            last_value: None,
            recipient: Recipient::new(recipient),
        };
        let msg = Subscribe { updater };
        self.interact(msg).await?;
        Ok(())
    }
}

pub struct ConfigUpdater {
    last_value: Option<Value>,
    recipient: Recipient<Value>,
}

impl ConfigUpdater {
    pub fn send_new_config(&mut self, value: Value) -> Result<()> {
        if self.last_value.as_ref() == Some(&value) {
            self.last_value = Some(value.clone());
        }
        self.recipient.send(value)
    }
}

pub struct TypedConfigListener<C: Config> {
    recipient: Recipient<UpdateConfigEvent<C>>,
}

impl<C> Sender<Value> for TypedConfigListener<C>
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
        if let Err(err) = agent.update_config(config, ctx).await {
            agent.fallback(err, ctx);
        }
        Ok(())
    }
}

pub struct Subscribe {
    updater: ConfigUpdater,
}

impl Request for Subscribe {
    type Response = ();
}

#[async_trait]
impl OnRequest<Subscribe> for Keeper {
    async fn on_request(&mut self, msg: Subscribe, _: &mut Self::Context) -> Result<()> {
        if let Some(value) = self.config.clone() {
            msg.updater.recipient.send(value).ok();
        }
        self.listeners.push(msg.updater);
        Ok(())
    }
}
