use crate::flow::Chat;
use crb::agent::{Agent, AgentSession};
use ice9_core::{Particle, SubstanceLinks};
use ui9_dui::Pub;

pub struct ChatParticle {
    chat: Pub<Chat>,
}

impl Particle for ChatParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            chat: Pub::unified(),
        }
    }
}

impl Agent for ChatParticle {
    type Context = AgentSession<Self>;
}
