use crb::core::{mpsc, Tag};
use crb::superagent::Drainer;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub fn to_drainer<M>(rx: mpsc::UnboundedReceiver<M>) -> Drainer<M>
where
    M: Tag,
{
    let stream = UnboundedReceiverStream::new(rx);
    Drainer::new(stream)
}
