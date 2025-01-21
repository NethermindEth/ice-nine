use crate::args::RunArgs;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Duty, Next, ToRecipient};
use crb::core::mpsc;
use crb::send::{Recipient, Sender};
use std::process::{ExitStatus, Stdio};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::select;
use tokio::time::{sleep, Duration};
use uiio::protocol::RecordDe;

#[derive(Debug)]
pub enum CommandEvent {
    Stdout(RecordDe),
    Terminated(Option<ExitStatus>),
}

pub enum CommandControl {}

pub struct CommandWatcher {
    command: String,
    arguments: Vec<String>,
    recipient: Recipient<CommandEvent>,
    exit_status: Option<ExitStatus>,
    receiver: mpsc::UnboundedReceiver<CommandControl>,
}

impl CommandWatcher {
    pub fn new(
        args: RunArgs,
        addr: impl ToRecipient<CommandEvent>,
        receiver: mpsc::UnboundedReceiver<CommandControl>,
    ) -> Self {
        Self {
            command: args.command,
            arguments: args.arguments,
            recipient: addr.to_recipient(),
            exit_status: None,
            receiver,
        }
    }
}

#[derive(Debug, Error)]
pub enum WatchError {
    #[error("No stdin")]
    NoStdin,
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

    fn end(&mut self) {
        let status = self.exit_status.take();
        let event = CommandEvent::Terminated(status);
        self.recipient.send(event).ok();
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
        let watch = Watch::from_child(child)?;
        Ok(Next::do_async(watch))
    }
}

impl CommandWatcher {
    fn process_stdout(&mut self, item: u8) {}
}

struct Watch {
    child: Child,
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
    stderr: Lines<BufReader<ChildStderr>>,

    child_terminated: bool,
    stdout_drained: bool,
    stderr_drained: bool,
}

impl Watch {
    fn from_child(mut child: Child) -> Result<Self> {
        let stdin = child.stdin.take().ok_or(WatchError::NoStdin)?;
        let stdout = child.stdout.take().ok_or(WatchError::NoStdout)?;
        let stderr = child.stderr.take().ok_or(WatchError::NoStderr)?;
        Ok(Watch {
            child,
            stdin,
            stdout: BufReader::new(stdout).lines(),
            stderr: BufReader::new(stderr).lines(),
            child_terminated: false,
            stdout_drained: false,
            stderr_drained: false,
        })
    }

    fn is_done(&self) -> bool {
        self.child_terminated && self.stdout_drained && self.stderr_drained
    }
}

#[async_trait]
impl DoAsync<Watch> for CommandWatcher {
    async fn repeat(&mut self, watch: &mut Watch) -> Result<Option<Next<Self>>> {
        select! {
            command = self.receiver.recv() => {
                if let Some(command) = command {
                    // TODO: Write to stdin of the process
                }
            }
            out_res = watch.stdout.next_line() => {
                match out_res {
                    Ok(None) | Err(_) => {
                        watch.stdout_drained = true;
                    }
                    Ok(Some(line)) => {
                        let record = serde_json::from_str(&line)?;
                        let event = CommandEvent::Stdout(record);
                        self.recipient.send(event)?;
                    }
                }
            }
            err_res = watch.stderr.next_line() => {
                match err_res {
                    Ok(None) | Err(_) => {
                        watch.stdout_drained = true;
                    }
                    Ok(Some(line)) => {
                        println!("{line}");
                        // TODO: Forward logs
                    }
                }
            }
            exit_res = watch.child.wait() => {
                watch.child_terminated = true;
                self.exit_status = Some(exit_res?);
            }
            _ = sleep(Duration::from_secs(1)) => {
                // Allow to be interrupted
            }
        }
        let state = watch.is_done().then(Next::done);
        Ok(state)
    }
}
