use crate::flow::Flow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};
use crb::core::watch;

pub enum Ported<F> {
    Loading,
    Loaded(F),
}

pub struct Player<F: Flow> {
    state: watch::Sender<Ported<F>>,
}

impl<F: Flow> Player<F> {
    pub fn new(state: watch::Sender<Ported<F>>) -> Self {
        Self { state }
    }
}

impl<F: Flow> Agent for Player<F> {
    type Context = AgentSession<Self>;
}

#[async_trait]
impl<F: Flow> OnEvent<F::Action> for Player<F> {
    async fn handle(&mut self, action: F::Action, _ctx: &mut Context<Self>) -> Result<()> {
        // TODO: Forward action to a recorder (hub)
        Ok(())
    }
}
