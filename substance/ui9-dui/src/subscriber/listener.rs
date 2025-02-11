use super::client::HubClient;
use super::{drainer, Act, LocalPlayer, Player, PlayerState, SubEvent};
use crate::flow::Flow;
use anyhow::{anyhow, Result};
use crb::agent::{RunAgent, StopRecipient};
use crb::core::mpsc;
use crb::runtime::InteractiveRuntime;
use crb::send::Sender;
use crb::superagent::Drainer;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: StopRecipient<Act<F>>,
    event_rx: Option<mpsc::UnboundedReceiver<SubEvent<F>>>,
}

impl<F: Flow> Listener<F> {
    pub fn local(fqn: Fqn) -> Self {
        Self::new::<LocalPlayer<F>>((), fqn)
    }

    pub fn new<P: Player<F>>(args: P::Args, fqn: Fqn) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let state = PlayerState {
            fqn,
            state_tx: None,
            event_tx,
        };
        let player = P::from_state(args, state);
        let agent = RunAgent::new(player);
        let player = agent.address().to_stop_address().to_stop_recipient();
        HubClient::add_player(agent);
        Self {
            player,
            event_rx: Some(event_rx),
        }
    }

    pub fn receiver(&mut self) -> Result<mpsc::UnboundedReceiver<SubEvent<F>>> {
        self.event_rx
            .take()
            .ok_or_else(|| anyhow!("Events stream (drainer) has taken already."))
    }

    pub fn events(&mut self) -> Result<Drainer<SubEvent<F>>> {
        self.receiver().map(drainer::from_mpsc)
    }

    pub fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.send(msg).ok();
    }
}
