use crate::flow::Flow;
use crb::agent::{Address};
use super::player::Player;

pub struct Listener<F: Flow> {
    player: Address<Player<F>>,
}
