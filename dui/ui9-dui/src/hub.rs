use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, OnEvent, Context};
use crb::superagent::{Supervisor, SupervisorSession};
use crb::runtime::Runtime;

pub struct Hub {
}

impl Supervisor for Hub {
    type GroupBy = ();
}

impl Agent for Hub {
    type Context = SupervisorSession<Self>;
}

pub struct Delegate {
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for Hub {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, ());
        Ok(())
    }
}
