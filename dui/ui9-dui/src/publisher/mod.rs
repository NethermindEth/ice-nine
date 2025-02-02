mod recorder;
mod server;
mod tracer;

pub use recorder::{EventFlow, Recorder, RecorderLink, UniRecorder};
pub use server::{HubServer, HubServerLink};
pub use tracer::{Tracer, TracerInfo};

use crate::flow::{Flow, Unified};
use crate::subscriber::Act;
use crb::core::mpsc;
use derive_more::{Deref, DerefMut};
use ui9::names::Fqn;

pub trait Publisher: Flow + Default {
    type Driver: From<Tracer<Self>>;
}

#[derive(Deref, DerefMut)]
pub struct Pub<P: Publisher> {
    driver: P::Driver,
}

impl<P: Publisher> Pub<P> {
    pub fn new(fqn: Fqn) -> Self {
        let state = P::default();
        let tracer = Tracer::<P>::new(fqn, state);
        Self {
            driver: P::Driver::from(tracer),
        }
    }

    pub fn unified() -> Self
    where
        P: Unified,
    {
        Self::new(P::fqn())
    }
}

#[derive(Deref, DerefMut)]
pub struct RecorderState<F: Flow> {
    #[deref]
    #[deref_mut]
    state: F,
    action_tx: mpsc::UnboundedSender<Act<F>>,
}
