use crate::app::App;
use anyhow::Result;
use crb::agent::{Address, Agent, AgentSession, DoSync, Next};
// use crossterm::event;

pub struct EventsDrainer {
    app: Address<App>,
}

impl EventsDrainer {
    pub fn new(app: Address<App>) -> Self {
        Self { app }
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
        /*
        let event = event::read()?;
        self.app.event(event)?;
        */
        Ok(None)
    }
}
