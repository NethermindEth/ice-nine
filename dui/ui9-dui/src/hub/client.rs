use crate::connector::Connector;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Duty, Context, Next};
use crb::superagent::{Supervisor, SupervisorSession};

pub struct HubClient {
}

impl Supervisor for HubClient {
    type GroupBy = ();
}

impl Agent for HubClient {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for HubClient {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let connector = Connector::new();
        ctx.spawn_agent(connector, ());
        Ok(Next::events())
    }
}
