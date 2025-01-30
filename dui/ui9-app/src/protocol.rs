use ui9_dui::subscriber::PortedState;
use ui9_dui::tracers::peer::Peer;

/// Ad event sent from `App` to `Ui`
pub enum UiEvent {
    SetState { peers: PortedState<Peer> },
    StateChanged,
}

/// Ad event sent from `Ui` to `App`
pub enum AppEvent {}
