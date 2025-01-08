use crate::keeper::{Keeper, KeeperClient};
use crate::particle::{Particle, ParticleSetup};
use anyhow::{anyhow as err, Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, Equip, InContext, Next, OnEvent, Standalone, Supervisor, SupervisorSession,
};
use derive_more::{Deref, DerefMut, From};
use std::any::type_name;
use std::marker::PhantomData;

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
    keeper: Option<KeeperClient>,
}

impl Substance {
    fn get_setup(&self) -> Result<ParticleSetup> {
        let keeper = self
            .keeper
            .clone()
            .ok_or_else(|| err!("Keeper is not started"))?;
        Ok(ParticleSetup { keeper })
    }
}

impl Standalone for Substance {}

impl Substance {
    pub fn new() -> Self {
        Self { keeper: None }
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
    Keeper,
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
        let keeper = ctx.spawn_agent(agent, Group::Keeper).equip();
        self.keeper = Some(keeper);
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
