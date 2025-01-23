use libp2p::request_response;
use serde::{Deserialize, Serialize};

pub type Event = request_response::Event<Request, Response>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(Vec<u8>);
