mod connector;
mod router;

pub use connector::{Connector, ConnectorLink};

use libp2p::StreamProtocol;

pub static PROTOCOL: StreamProtocol = StreamProtocol::new("/ui9-flow");
