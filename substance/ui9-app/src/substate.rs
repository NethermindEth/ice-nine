use crate::ported::{Ported, PortedExt};
use derive_more::with_trait::{Deref, DerefMut};
use ui9_dui::{Listener, State, Sub, Subscriber, Unified};

#[derive(Deref, DerefMut)]
pub struct SubState<F: Subscriber> {
    pub sub: Sub<F>,
    #[deref]
    #[deref_mut]
    pub state: State<Ported<F>>,
}

impl<F: Subscriber> SubState<F> {
    pub fn new_local_unified() -> Self
    where
        F: Unified,
        F::Driver: DerefMut<Target = Listener<F>>,
    {
        let mut sub = Sub::<F>::local_unified();
        let state = sub
            .ported_state()
            .expect("A state always available for a newly created subscribtion");
        Self { sub, state }
    }
}
