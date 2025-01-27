use crate::subscriber::Ported;
use crb::core::watch;

pub struct DeltaWatcher<F> {
    state: watch::Receiver<Ported<F>>,
}

impl<F> DeltaWatcher<F> {}
