use crate::config::OpenAIConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next};
use ice_nine_core::{KeeperClient, Particle, ParticleSetup};

pub struct OpenAIParticle {
    keeper: KeeperClient,
}

impl Particle for OpenAIParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
            keeper: setup.keeper,
        }
    }
}

impl Agent for OpenAIParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Configure)
    }
}

struct Configure;

#[async_trait]
impl DoAsync<Configure> for OpenAIParticle {
    async fn once(&mut self, _: &mut Configure) -> Result<Next<Self>> {
        println!("Configuring...");
        let config: OpenAIConfig = self.keeper.get_config().await?;
        Ok(Next::events())
    }
}
