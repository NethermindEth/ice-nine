use crate::flow::Flow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};

pub struct Relay<F: Flow> {
    state: F,
}

impl<F: Flow> Relay<F> {
    pub fn new(state: F) -> Self {
        Self { state }
    }
}

impl<F: Flow> Agent for Relay<F> {
    type Context = AgentSession<Self>;
}

#[async_trait]
impl<F: Flow> OnEvent<F::Event> for Relay<F> {
    async fn handle(&mut self, event: F::Event, ctx: &mut Context<Self>) -> Result<()> {
        self.state.apply(event);
        // TODO: Distirbute events to subscribers...
        Ok(())
    }
}
