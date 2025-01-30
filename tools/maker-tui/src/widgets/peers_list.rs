use crate::widgets::{Component, Reason};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_dui::subscriber::PortedState;
use ui9_dui::tracers::peer::Peer;

pub struct PeerList {
    peers: Option<PortedState<Peer>>,
}

impl PeerList {
    pub fn new() -> Self {
        Self { peers: None }
    }

    pub fn set_state(&mut self, state: PortedState<Peer>) {
        self.peers = Some(state);
    }
}

impl Component for PeerList {
    fn title(&self) -> Option<&str> {
        Some("Peers")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let state = self.peers.as_ref().ok_or("Connecting to a hub...")?;
        let peers = state.borrow();
        let peers_state = peers.loaded().ok_or("Loading peers list...")?;
        if peers_state.peers.is_empty() {
            return Err("No peers connected yet".into());
        }
        // Convert peers to ListItems
        let items: Vec<ListItem> = peers_state
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
        let mut list = List::new(items);

        // Render the List widget
        list.render(area, buf);
        Ok(())
    }
}
