use crate::app::App;
use anyhow::Result;
use crb::agent::{Address, Agent, AgentSession, DoSync, Next};

pub struct EventsDrainer {
    _app: Address<App>,
}

impl EventsDrainer {
    pub fn new(app: Address<App>) -> Self {
        Self { _app: app }
    }
}

impl Agent for EventsDrainer {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_sync(())
    }
}

impl DoSync for EventsDrainer {
    fn repeat(&mut self, _: &mut ()) -> Result<Option<Next<Self>>> {
        Ok(None)
    }
}
