use crb::agent::{Agent, AgentSession, Next};
use ice_nine_core::{ParticleSetup, Particle};

pub struct StdioParticle {
}

impl Particle for StdioParticle {
    fn construct(setup: ParticleSetup) -> Self {
        Self {
        }
    }
}

impl Agent for StdioParticle {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}
