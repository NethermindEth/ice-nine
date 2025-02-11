use crb::agent::{Agent, AgentSession};
use ui9_dui::{Sub, Subscriber};

pub struct Shadow<F: Subscriber> {
    sub: Sub<F>,
}

impl<F: Subscriber> Agent for Shadow<F> {
    type Context = AgentSession<Self>;
}
