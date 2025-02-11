use super::shadow::Shadow;
use anyhow::Result;
use crb::agent::Address;
use crb::superagent::Drainer;
use futures::StreamExt;
use std::ops::DerefMut;
use ui9::names::Fqn;
use ui9_dui::{Listener, Ported, State, Sub, SubEvent, Subscriber};

pub trait Component: Default {
    type Flow: Subscriber<Driver: DerefMut<Target = Listener<Self::Flow>>>;
}

pub struct ComponentWidget<C: Component> {
    component: C,
    sub: Sub<C::Flow>,
    ported_state: State<Ported<C::Flow>>,
}

impl<C: Component> ComponentWidget<C> {
    pub fn create(fqn: Fqn) -> Result<Self> {
        let component = C::default();
        let mut sub = Sub::<C::Flow>::local(fqn);
        let ported_state = sub.ported_state()?;
        Ok(Self {
            component,
            sub,
            ported_state,
        })
    }
}
