use crate::relay::RemotePlayer;
use libp2p::PeerId;
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Sub, Subscriber, Unified};

pub trait RemoteExt {
    fn remote(peer: PeerId, fqn: Fqn) -> Self;
}

impl<F: Flow> RemoteExt for Listener<F> {
    fn remote(peer: PeerId, fqn: Fqn) -> Self {
        Self::new::<RemotePlayer<F>>(peer, fqn)
    }
}

impl<P: Subscriber> RemoteExt for Sub<P> {
    fn remote(peer: PeerId, fqn: Fqn) -> Self {
        let listener = Listener::<P>::remote(peer, fqn);
        Self::new(listener)
    }
}

pub trait RemoteUnifiedExt<U: Unified> {
    fn remote_unified(peer: PeerId) -> Self;
}

impl<P: Subscriber + Unified> RemoteUnifiedExt<P> for Sub<P> {
    fn remote_unified(peer: PeerId) -> Self {
        Self::remote(peer, P::fqn())
    }
}
