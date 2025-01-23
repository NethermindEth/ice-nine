use crate::app::App;
use anyhow::Result;
use crb::agent::{Address, Agent, AgentSession, DoSync, Next, ToAddress};

pub struct EventsDrainer {
    _app: Address<App>,
}

impl EventsDrainer {
    pub fn new(app: impl ToAddress<App>) -> Self {
        Self {
            _app: app.to_address(),
        }
    }
}

impl Agent for EventsDrainer {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_sync(())
    }
}

impl DoSync for EventsDrainer {
    fn repeat(&mut self, _: &mut ()) -> Result<Option<Next<Self>>> {
        Ok(None)
    }
}
