use super::shadow::Shadow;
use crb::agent::Address;
use crb::superagent::Drainer;
use futures::StreamExt;
use ui9_dui::{State, Sub, SubEvent, Subscriber};

pub trait Component: Default {
    type Flow: Subscriber;
}

pub struct ComponentWidget<C: Component> {
    component: C,
    sub: Sub<C::Flow>,
    events: Drainer<SubEvent<C::Flow>>,
    state: Option<State<C::Flow>>,
}

impl<C: Component> ComponentWidget<C> {}
