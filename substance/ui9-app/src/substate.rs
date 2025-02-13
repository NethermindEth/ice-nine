use crate::ported::{Ported, PortedExt};
use anyhow::Result;
use crb::core::watch;
use derive_more::{Deref, DerefMut};
use ui9_dui::{Flow, Listener, State, Sub, Subscriber, Unified};

#[derive(Deref, DerefMut)]
pub struct SubState<F: Subscriber> {
    sub: Sub<F>,
    #[deref]
    #[deref_mut]
    state: State<Ported<F>>,
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

    pub fn state<'a>(&'a self) -> Result<Ref<'a, F>> {
        Ok(Ref {
            outer: self.state.borrow(),
            inner: None,
        })
    }
}

pub struct Ref<'a, F> {
    outer: watch::Ref<'a, Ported<F>>,
    inner: Option<watch::Ref<'a, F>>,
}

impl<'a, F: Flow> Ref<'a, F> {
    fn unpack(&'a mut self) -> Option<watch::Ref<'a, F>> {
        let state = self.outer.state()?;
        Some(state)
    }

    fn unpack_back(&'a mut self) -> Option<()> {
        let state = self.outer.state()?;
        self.inner = Some(state);
        Some(())
    }

    fn into_full(self) -> RefFull<'a, F> {
        RefFull {
            outer: self.outer,
            inner: self.inner.unwrap(),
        }
    }
}

pub struct RefFull<'a, F> {
    outer: watch::Ref<'a, Ported<F>>,
    inner: watch::Ref<'a, F>,
}
