use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Standalone, Supervisor, SupervisorSession, InContext, Next};
use crate::keeper::{Keeper, KeeperAddress};

pub struct Substance {
    keeper: Option<KeeperAddress>,
}

impl Standalone for Substance {}

impl Substance {
    pub fn new() -> Self {
        Self {
            keeper: None,
        }
    }
}

impl Agent for Substance {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::in_context(Configure)
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Group {
    Particles,
}

impl Supervisor for Substance {
    type GroupBy = Group;
}

struct Configure;

#[async_trait]
impl InContext<Configure> for Substance {
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let agent = Keeper::new();
        let addr = ctx.spawn_agent(agent, Group::Particles);
        let keeper = KeeperAddress::from(addr);
        self.keeper = Some(keeper);
        Ok(Next::process())
    }
}
