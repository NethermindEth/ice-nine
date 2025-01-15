use super::{Config, Keeper, KeeperLink};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, MessageFor, ToAddress};
use crb::core::UniqueId;
use crb::send::{Recipient, Sender};
use crb::superagent::{ManageSubscription, SubscribeExt, Subscription};
use std::any::type_name;
use std::marker::PhantomData;
use toml::{Table, Value};

#[async_trait]
pub trait UpdateConfig<C: Config>: Agent {
    async fn update_config(&mut self, config: C, ctx: &mut Context<Self>) -> Result<()>;

    fn fallback(&mut self, err: Error, _ctx: &mut Context<Self>) {
        log::error!("Can't load the config {}: {err}", type_name::<C>());
    }
}

impl KeeperLink {
    /// Subscribe to live configuration updates.
    pub async fn live_config_updates<A, C>(
        &self,
        address: impl ToAddress<A>,
        namespace: String,
    ) -> Result<()>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let recipient = TypedConfigListener {
            recipient: address.to_address().sender(),
        };
        let updates = ConfigSegmentUpdates {
            namespace,
            recipient: Recipient::new(recipient),
        };
        self.subscribe(updates).await?;
        Ok(())
    }
}

pub struct NewConfigSegment(pub Value);

pub struct ConfigSegmentUpdates {
    namespace: String,
    // TODO: Keep `Arc` with a default value here
    recipient: Recipient<NewConfigSegment>,
}

impl Subscription for ConfigSegmentUpdates {
    type State = Value;
}

pub struct Subscriber {
    last_value: Option<Value>,
}

impl Keeper {
    pub fn distribute(&mut self) {
        for (id, subscriber) in &mut self.subscribers {
            let value = self.config.get_config(&id.namespace);
            if subscriber.last_value.as_ref() == Some(&value) {
                subscriber.last_value = Some(value.clone());
            }
            id.recipient.send(NewConfigSegment(value.clone()));
        }
    }
}

#[async_trait]
impl ManageSubscription<ConfigSegmentUpdates> for Keeper {
    async fn subscribe(
        &mut self,
        sub_id: UniqueId<ConfigSegmentUpdates>,
        ctx: &mut Context<Self>,
    ) -> Result<Value> {
        let subscriber = Subscriber { last_value: None };
        let value = self.config.get_config(&sub_id.namespace);
        self.subscribers.insert(sub_id, subscriber);
        Ok(value)
    }

    async fn unsubscribe(
        &mut self,
        sub_id: UniqueId<ConfigSegmentUpdates>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        self.subscribers.remove(&sub_id);
        Ok(())
    }
}

pub struct TypedConfigListener<C: Config> {
    recipient: Recipient<UpdateConfigEvent<C>>,
}

impl<C> Sender<NewConfigSegment> for TypedConfigListener<C>
where
    C: Config,
{
    fn send(&self, value: NewConfigSegment) -> Result<()> {
        let event = UpdateConfigEvent {
            _type: PhantomData::<C>,
            value: value.0,
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
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut Context<A>) -> Result<()> {
        let result = match self.value.try_into() {
            Ok(config) => agent.update_config(config, ctx).await,
            Err(err) => {
                let ns = C::NAMESPACE;
                log::error!("Can't parse the section 'particle.{ns}.config': {err}");
                Err(err.into())
            }
        };
        if let Err(err) = result {
            agent.fallback(err, ctx);
        }
        Ok(())
    }
}
