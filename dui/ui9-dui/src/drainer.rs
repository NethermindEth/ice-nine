use crate::flex::FlexCodec;
use crate::protocol::Message;
use anyhow::{Error, Result};
use crb::superagent::Drainer;
use futures::Sink;
use futures::StreamExt;
use libp2p::Stream;
use std::pin::Pin;
use tokio_util::codec::Framed;
use tokio_util::compat::FuturesAsyncReadCompatExt;

pub type MessageSink = Pin<Box<dyn Sink<Message, Error = Error> + Send>>;

pub fn to_drainer(stream: Stream) -> (Drainer<Result<Message>>, MessageSink) {
    let stream = stream.compat();
    let codec = FlexCodec::<Message>::new();
    let framed = Framed::new(stream, codec);
    let (writer, reader) = framed.split();
    let drainer = Drainer::new(reader);
    (drainer, Box::pin(writer))
}
