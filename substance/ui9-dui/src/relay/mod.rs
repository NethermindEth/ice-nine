mod connector;
mod drainer;
mod flex;
mod node;
mod protocol;
mod relay_player;
mod remote_player;
mod router;

pub use connector::{Connector, ConnectorLink};
pub use node::MeshNode;
pub use remote_player::RemotePlayer;

use libp2p::StreamProtocol;

pub static PROTOCOL: StreamProtocol = StreamProtocol::new("/ui9-flow");
