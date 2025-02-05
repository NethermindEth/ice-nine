use anyhow::{Error, Result};
use crb::superagent::Drainer;
use futures::TryStreamExt;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

pub fn input() -> Drainer<Result<String>> {
    let reader = BufReader::new(stdin());
    let lines = reader.lines();
    let stream = LinesStream::new(lines).map_err(Error::from);
    Drainer::new(stream)
}
