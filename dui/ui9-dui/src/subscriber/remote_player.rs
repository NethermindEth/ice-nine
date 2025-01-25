use super::Ported;
use crate::Flow;
use crb::core::watch;
use ui9::names::Fqn;

pub struct RemovePlayer<F: Flow> {
    fqn: Fqn,
    state: watch::Sender<Ported<F>>,
}

impl<F: Flow> RemovePlayer<F> {
    pub fn new(fqn: Fqn, state: watch::Sender<Ported<F>>) -> Self {
        Self { fqn, state }
    }
}
