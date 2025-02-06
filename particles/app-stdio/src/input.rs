use anyhow::{Error, Result};
use crb::superagent::Drainer;
use futures::{FutureExt, TryStreamExt};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

pub fn lines() -> Drainer<Result<String>> {
    let reader = BufReader::new(stdin());
    let lines = reader.lines();
    let stream = LinesStream::new(lines).map_err(Error::from);
    Drainer::new(stream)
}

pub async fn next_line() -> Result<String> {
    let reader = BufReader::new(stdin());
    let mut lines = reader.lines();
    let line = lines.next_line().await?
        .ok_or_else(|| Error::msg("Stdin closed"))?;
    Ok(line)
}

pub struct CtrlC;

pub fn signals() -> Drainer<CtrlC> {
    let stream = futures::stream::unfold((), |init| {
        tokio::signal::ctrl_c().map(move |res| res.ok().map(|_| (CtrlC, init)))
    });
    Drainer::new(stream)
}
