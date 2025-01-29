use crate::app::AppTui;
use anyhow::Result;
use crb::agent::{Address, Agent, AgentSession, DoSync, Next, ToAddress};
use crossterm::event;
use std::time::Duration;

pub struct EventsDrainer {
    app: Address<AppTui>,
}

impl EventsDrainer {
    pub fn new(app: impl ToAddress<AppTui>) -> Self {
        Self {
            app: app.to_address(),
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
        if event::poll(Duration::from_millis(1_000))? {
            let event = event::read()?;
            self.app.event(event)?;
        }
        Ok(None)
    }
}
