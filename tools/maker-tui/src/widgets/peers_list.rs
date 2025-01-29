use crate::widgets::smart_widget::Component;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_dui::subscriber::State;
use ui9_dui::tracers::peer::Peer;

pub struct PeerList {
    peers: State<Peer>,
}

impl Component for PeerList {
    fn render(&self, area: Rect, buf: &mut Buffer) -> Option<()> {
        let peers = self.peers.borrow();
        let peers_state = peers.loaded()?;
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
        Some(())
    }
}
