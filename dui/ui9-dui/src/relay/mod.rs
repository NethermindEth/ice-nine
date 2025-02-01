pub mod router;

use libp2p::StreamProtocol;

pub static PROTOCOL: StreamProtocol = StreamProtocol::new("/ui9-flow");
