use super::connector::ConnectorLink;
use super::relay_player::RelayPlayer;
use super::PROTOCOL;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{Drainer, Supervisor, SupervisorSession};
use libp2p::{PeerId, Stream};

pub struct Router {
    connector: ConnectorLink,
}

impl Router {
    pub fn new(connector: ConnectorLink) -> Self {
        Self { connector }
    }
}

impl Supervisor for Router {
    type GroupBy = ();
}

impl Agent for Router {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Router {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut control = self.connector.get_control().await?;
        let streams = control.accept(PROTOCOL.clone())?;
        let drainer = Drainer::new(streams);
        ctx.assign(drainer, (), ());
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<(PeerId, Stream)> for Router {
    async fn handle(&mut self, event: (PeerId, Stream), ctx: &mut Context<Self>) -> Result<()> {
        let relay = RelayPlayer::new(event.1);
        ctx.spawn_agent(relay, ());
        Ok(())
    }
}
