use super::particle::{Particle, ParticleSetup};
use super::SubstanceLinks;
use crate::keeper::Keeper;
use crate::router::ReasoningRouter;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Equip, Next, OnEvent, Standalone};
use crb::core::Slot;
use crb::superagent::{InteractExt, OnRequest, Request, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use std::any::type_name;
use std::marker::PhantomData;

#[derive(Deref, DerefMut, From, Clone)]
pub struct SubstanceLink {
    address: Address<Substance>,
}

impl SubstanceLink {
    pub fn add_particle<P: Particle>(&self) -> Result<()> {
        let msg = AddParticle::<P> { _type: PhantomData };
        self.address.event(msg)
    }

    pub async fn be_particle(&self) -> Result<ParticleSetup> {
        self.address.interact(BeParticle).await.map_err(Error::from)
    }
}

pub struct Substance {
    links: Slot<SubstanceLinks>,
}

impl Substance {
    pub fn arise() -> SubstanceLink {
        Self::new().spawn().equip()
    }

    fn get_setup(&mut self) -> Result<ParticleSetup> {
        let links = self.links.get_mut()?.clone();
        Ok(ParticleSetup { links })
    }
}

impl Standalone for Substance {}

impl Substance {
    pub fn new() -> Self {
        Self {
            links: Slot::empty(),
        }
    }
}

impl Agent for Substance {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
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
impl Duty<Configure> for Substance {
    async fn handle(&mut self, _: Configure, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let agent = Keeper::new();
        let keeper = ctx.spawn_agent(agent, Group::Services).equip();

        let agent = ReasoningRouter::new();
        let router = ctx.spawn_agent(agent, Group::Services).equip();

        let links = SubstanceLinks { keeper, router };
        self.links.fill(links)?;

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
    async fn handle(&mut self, _: AddParticle<P>, ctx: &mut Context<Self>) -> Result<()> {
        log::info!("Add particle: {}", type_name::<P>());
        let setup = self.get_setup()?;
        let agent = P::construct(setup);
        let _addr = ctx.spawn_agent(agent, Group::Particles);
        Ok(())
    }
}

struct BeParticle;

impl Request for BeParticle {
    type Response = ParticleSetup;
}

#[async_trait]
impl OnRequest<BeParticle> for Substance {
    async fn on_request(
        &mut self,
        _: BeParticle,
        _ctx: &mut Context<Self>,
    ) -> Result<ParticleSetup> {
        self.get_setup()
    }
}
