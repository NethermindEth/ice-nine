use crate::keeper::{Keeper, KeeperClient};
use crate::particle::{Particle, ParticleSetup};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, Equip, InContext, Next, OnEvent, Standalone, Supervisor, SupervisorSession,
};
use crb::core::Slot;
use derive_more::{Deref, DerefMut, From};
use std::any::type_name;
use std::marker::PhantomData;
use crate::conversation_router::{ConversationRouterClient, ConversationRouter};

#[derive(Deref, DerefMut, From, Clone)]
pub struct SubstanceClient {
    address: Address<Substance>,
}

impl SubstanceClient {
    pub fn add_particle<P: Particle>(&self) -> Result<()> {
        let msg = AddParticle::<P> { _type: PhantomData };
        self.address.event(msg)
    }
}

pub struct Substance {
    keeper: Slot<KeeperClient>,
    conversation_router: Slot<ConversationRouterClient>,
}

impl Substance {
    fn get_setup(&mut self) -> Result<ParticleSetup> {
        let keeper = self.keeper.get_mut()?.clone();
        let conversation_router = self.conversation_router.get_mut()?.clone();
        Ok(ParticleSetup {
            keeper,
            conversation_router,
        })
    }
}

impl Standalone for Substance {}

impl Substance {
    pub fn new() -> Self {
        Self {
            keeper: Slot::empty(),
            conversation_router: Slot::empty(),
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
    Services,
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
        let addr = ctx.spawn_agent(agent, Group::Services);
        self.keeper.fill(addr.equip())?;

        let agent = ConversationRouter::new();
        let addr = ctx.spawn_agent(agent, Group::Services);
        self.conversation_router.fill(addr.equip())?;

        Ok(Next::events())
    }
}

struct AddParticle<P> {
    _type: PhantomData<P>,
}

#[async_trait]
impl<P> OnEvent<AddParticle<P>> for Substance
where
    P: Particle,
{
    type Error = Error;

    async fn handle(&mut self, _: AddParticle<P>, ctx: &mut Self::Context) -> Result<()> {
        log::info!("Add particle: {}", type_name::<P>());
        let setup = self.get_setup()?;
        let agent = P::construct(setup);
        let _addr = ctx.spawn_agent(agent, Group::Particles);
        Ok(())
    }
}
