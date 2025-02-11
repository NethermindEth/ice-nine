use ui9_dui::subscriber::State;
use ui9_net::tracers::peer::Peer;

/// Ad event sent from `App` to `Ui`
pub enum UiEvent {
    SetState { peers: State<Peer> },
    StateChanged,
}

/// Ad event sent from `Ui` to `App`
pub enum AppEvent {}
