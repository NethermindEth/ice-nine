use anyhow::Error;
use crb::core::watch;
use thiserror::Error;
use ui9_dui::{Flow, Listener, State, SubEvent};

#[derive(Debug, Error)]
#[error("Loading...")]
pub struct Loading;

#[derive(Debug)]
pub enum Ported<F> {
    Loading,
    Actual(State<F>),
    Spoiled(State<F>),
}

impl<F: Flow> Ported<F> {
    pub fn get_state(&self) -> Option<watch::Ref<F>> {
        match self {
            Self::Loading => None,
            Self::Actual(state) => Some(state.borrow()),
            Self::Spoiled(state) => Some(state.borrow()),
        }
    }

    pub fn state(&self) -> Result<watch::Ref<F>, Loading> {
        self.get_state().ok_or(Loading)
    }
}

pub trait PortedExt<F> {
    fn ported_state(&mut self) -> Result<State<Ported<F>>, Error>;
}

impl<F: Flow> PortedExt<F> for Listener<F> {
    fn ported_state(&mut self) -> Result<State<Ported<F>>, Error> {
        let mut rx = self.receiver()?;
        let (state, state_tx) = State::new(Ported::Loading);
        crb::core::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    SubEvent::State(state) => {
                        state_tx.send(Ported::Actual(state))?;
                    }
                    SubEvent::Event(_) => {}
                    SubEvent::Lost => {
                        state_tx.send_modify(|ported| {
                            let mut swapper = Ported::Loading;
                            std::mem::swap(&mut swapper, ported);
                            if let Ported::Actual(state) = swapper {
                                swapper = Ported::Spoiled(state);
                            }
                            std::mem::swap(&mut swapper, ported);
                        });
                    }
                }
            }
            Ok::<(), Error>(())
        });
        Ok(state)
    }
}
