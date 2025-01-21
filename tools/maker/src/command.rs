use crate::args::RunArgs;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Duty, Next, ToRecipient};
use crb::send::Recipient;
use std::process::{ExitStatus, Stdio};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{Child, ChildStderr, ChildStdout, Command};
use tokio::select;
use tokio::time::{sleep, Duration};
use uiio::protocol::RecordDe;

pub enum UiioEvent {
    Stdout(RecordDe),
}

pub struct CommandWatcher {
    command: String,
    arguments: Vec<String>,
    recipient: Recipient<UiioEvent>,
}

impl CommandWatcher {
    pub fn new(args: RunArgs, addr: impl ToRecipient<UiioEvent>) -> Self {
        Self {
            command: args.command,
            arguments: args.arguments,
            recipient: addr.to_recipient(),
        }
    }
}

#[derive(Debug, Error)]
pub enum WatchError {
    #[error("No stdout")]
    NoStdout,
    #[error("No stderr")]
    NoStderr,
}

impl Agent for CommandWatcher {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for CommandWatcher {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut child = Command::new(&self.command)
            .args(&self.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().ok_or(WatchError::NoStdout)?;
        let stderr = child.stderr.take().ok_or(WatchError::NoStderr)?;

        let stdout = BufReader::new(stdout).lines();
        let stderr = BufReader::new(stderr).lines();

        let watch = Watch {
            child,
            stdout,
            stderr,

            exit_status: None,
            stdout_drained: false,
            stderr_drained: false,
        };
        Ok(Next::do_async(watch))
    }
}

impl CommandWatcher {
    fn process_stdout(&mut self, item: u8) {}
}

struct Watch {
    child: Child,
    stdout: Lines<BufReader<ChildStdout>>,
    stderr: Lines<BufReader<ChildStderr>>,

    exit_status: Option<ExitStatus>,
    stdout_drained: bool,
    stderr_drained: bool,
}

impl Watch {
    fn is_done(&self) -> bool {
        self.exit_status.is_some() && self.stdout_drained && self.stderr_drained
    }
}

#[async_trait]
impl DoAsync<Watch> for CommandWatcher {
    async fn repeat(&mut self, watch: &mut Watch) -> Result<Option<Next<Self>>> {
        select! {
            out_res = watch.stdout.next_line() => {
                match out_res {
                    Ok(None) | Err(_) => {
                        watch.stdout_drained = true;
                    }
                    Ok(Some(line)) => {
                        let record = serde_json::from_str(&line)?;
                        let event = UiioEvent::Stdout(record);
                    }
                }
            }
            err_res = watch.stderr.next_line() => {
            }
            _ = watch.child.wait() => {
            }
            _ = sleep(Duration::from_secs(1)) => {
                // Allow to be interrupted
            }
        }
        let state = watch.is_done().then(Next::done);
        Ok(state)
    }
}
