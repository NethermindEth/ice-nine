use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context};
use crb::superagent::{OnRequest, Request};
use tokio::io::{stdin, AsyncBufReadExt, BufReader, Lines, Stdin};

pub type Line = Option<String>;

pub struct StdinDrainer {
    lines: Lines<BufReader<Stdin>>,
}

impl StdinDrainer {
    pub fn new() -> Self {
        Self {
            lines: BufReader::new(stdin()).lines(),
        }
    }
}

impl Agent for StdinDrainer {
    type Context = AgentSession<Self>;
}

pub struct ReadLine;

impl Request for ReadLine {
    type Response = Line;
}

#[async_trait]
impl OnRequest<ReadLine> for StdinDrainer {
    async fn on_request(&mut self, _: ReadLine, _: &mut Context<Self>) -> Result<Line> {
        self.lines.next_line().await.map_err(Error::from)
    }
}
