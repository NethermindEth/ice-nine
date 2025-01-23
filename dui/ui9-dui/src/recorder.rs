use crate::flow::Flow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};

pub struct Recorder<F: Flow> {
    state: F,
}

impl<F: Flow> Recorder<F> {
    pub fn new(state: F) -> Self {
        Self { state }
    }
}

impl<F: Flow> Agent for Recorder<F> {
    type Context = AgentSession<Self>;
}

#[async_trait]
impl<F: Flow> OnEvent<F::Event> for Recorder<F> {
    async fn handle(&mut self, event: F::Event, _ctx: &mut Context<Self>) -> Result<()> {
        self.state.apply(event);
        // TODO: Distirbute events to subscribers...
        Ok(())
    }
}
