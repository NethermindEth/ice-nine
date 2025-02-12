use crate::widgets::{Component, Reason};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_app::{Ported, PortedExt};
use ui9_dui::{State, Sub};
use ui9_net::tracers::peer::Peer;

pub struct PeerList {
    peers: Sub<Peer>,
    state: State<Ported<Peer>>,
}

impl PeerList {
    pub fn new() -> Self {
        let mut peers = Sub::<Peer>::local_unified();
        let state = peers.ported_state().unwrap();
        Self { peers, state }
    }
}

impl Component for PeerList {
    fn title(&self) -> Option<&str> {
        Some("Peers")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state_result()?;

        if state.peers.is_empty() {
            return Err("No peers connected yet".into());
        }
        // Convert peers to ListItems
        let items: Vec<ListItem> = state
            .peers
            .iter()
            .map(|(peer, _)| {
                ListItem::new(Line::from(vec![Span::styled(
                    peer.to_string(),
                    Style::default().fg(Color::Yellow),
                )]))
            })
            .collect();

        // Create a List widget
        let list = List::new(items);

        // Render the List widget
        list.render(area, buf);
        Ok(())
    }
}
