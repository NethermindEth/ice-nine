use super::{Config, Keeper, KeeperLink};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Agent, MessageFor, OnEvent};
use crb::send::{Recipient, Sender};
use crb::superagent::{OnRequest, Request};
use std::any::type_name;
use std::marker::PhantomData;
use toml::Value;

#[async_trait]
pub trait UpdateConfig<C: Config>: Agent {
    async fn update_config(&mut self, config: C, ctx: &mut Self::Context) -> Result<()>;

    fn fallback(&mut self, err: Error, _ctx: &mut Self::Context) {
        log::error!("Can't load the config {}: {err}", type_name::<C>());
    }
}

impl KeeperLink {
    /// Subscribe to live configuration updates.
    pub async fn subscribe<A, C>(&self, address: Address<A>, namespace: String) -> Result<()>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let recipient = TypedConfigListener {
            recipient: address.sender(),
        };
        let updater = ConfigUpdater {
            namespace,
            last_value: None,
            recipient: Recipient::new(recipient),
        };
        let msg = Subscribe { updater };
        self.interact(msg).await?;
        Ok(())
    }
}

pub struct ConfigUpdater {
    namespace: String,
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

#[async_trait]
impl OnEvent<Value> for Keeper {
    async fn handle(&mut self, value: Value, _ctx: &mut Self::Context) -> Result<()> {
        println!("Config updated: {:?}", value);
        self.config = Some(value.clone());
        for updater in &mut self.listeners {
            if let Some(value) = get_config(&value, &updater.namespace) {
                updater.send_new_config(value).ok();
            }
        }
        Ok(())
    }
}

fn get_config(value: &Value, namespace: &str) -> Option<Value> {
    value
        .get("particle")?
        .get(namespace)?
        .get("config")
        .cloned()
}
