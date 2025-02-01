use crb::core::{mpsc, Tag};
use crb::superagent::Drainer;
use tokio_stream::wrappers::UnboundedReceiverStream;
use crate::flex::FlexCodec;
use crate::protocol::Ui9Message;
use anyhow::{Error, Result};
use futures::Sink;
use futures::StreamExt;
use libp2p::Stream;
use std::pin::Pin;
use tokio_util::codec::Framed;
use tokio_util::compat::FuturesAsyncReadCompatExt;


pub fn from_mpsc<M>(rx: mpsc::UnboundedReceiver<M>) -> Drainer<M>
where
    M: Tag,
{
    let stream = UnboundedReceiverStream::new(rx);
    Drainer::new(stream)
}

pub type MessageSink = Pin<Box<dyn Sink<Ui9Message, Error = Error> + Send>>;

pub fn from_stream(stream: Stream) -> (Drainer<Result<Ui9Message>>, MessageSink) {
    let stream = stream.compat();
    let codec = FlexCodec::<Ui9Message>::new();
    let framed = Framed::new(stream, codec);
    let (writer, reader) = framed.split();
    let drainer = Drainer::new(reader);
    (drainer, Box::pin(writer))
}
