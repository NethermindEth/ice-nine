mod connector;
pub mod flex;
mod router;
mod relay_player;
mod remote_player;
mod drainer;

pub use connector::{Connector, ConnectorLink};
pub use remote_player::RemotePlayer;

use libp2p::StreamProtocol;

pub static PROTOCOL: StreamProtocol = StreamProtocol::new("/ui9-flow");
