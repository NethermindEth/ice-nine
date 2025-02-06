use crate::input::{self, CtrlC};
use crate::output::{IoControl, RATE};
use crate::queue::Queue;
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::time::Duration;
use crb::core::Slot;
use crb::superagent::{Interval, StreamSession, Supervisor};
use ice9_core::{Particle, SubstanceLinks};
use n9_control_chat::{Chat, ChatEvent, Role};
use std::collections::VecDeque;
use ui9_dui::tracers::live::Live;
use ui9_dui::{State, Sub, SubEvent};

pub struct StdioApp {
    substance: SubstanceLinks,
    io_control: Slot<IoControl>,

    chat: Sub<Chat>,
    state: Option<State<Chat>>,

    live: Sub<Live>,

    prompts: VecDeque<String>,

    queue: Queue,
    thinking: Option<String>,
    interval: Interval<Tick>,
    asking: bool,
}

impl Particle for StdioApp {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            io_control: Slot::empty(),
            chat: Sub::unified(None),
            state: None,

            live: Sub::unified(None),

            prompts: VecDeque::new(),

            queue: Queue::new(),
            thinking: None,
            interval: Interval::new(Tick, Duration::from_millis(RATE)),
            asking: false,
        }
    }
}

impl Supervisor for StdioApp {
    type GroupBy = ();
}

impl Agent for StdioApp {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for StdioApp {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.io_control.fill(IoControl::new()?)?;
        let io_control = self.io_control.get_mut()?;
        io_control.writeln(&"Nine".blue().to_string()).await?;

        self.interval.add_listener(&ctx);

        self.queue.add_message("Loading the state...");
        self.interval.start();

        ctx.consume(self.chat.events()?);
        ctx.consume(self.live.events()?);
        ctx.consume(input::lines());
        ctx.consume(input::signals());
        Ok(Next::events())
    }
}

#[derive(Clone)]
struct Tick;

#[async_trait]
impl OnEvent<Tick> for StdioApp {
    async fn handle(&mut self, _: Tick, ctx: &mut Context<Self>) -> Result<()> {
        if !self.asking {
            let io_control = self.io_control.get_mut()?;
            if let Some(reason) = self.queue.pick_next() {
                io_control.render_progress(reason).await?;
            } else {
                self.interval.stop();
                io_control.clear_line().await?;
                if let Some(prompt) = self.prompts.pop_front() {
                    ctx.do_next(Next::do_async(ProcessingPrompt { prompt }));
                } else {
                    io_control.write(">> ").await?;
                    self.asking = true;
                }
            }
        }
        Ok(())
    }
}

struct ProcessingPrompt {
    prompt: String,
}

#[async_trait]
impl DoAsync<ProcessingPrompt> for StdioApp {
    async fn handle(&mut self, msg: ProcessingPrompt, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let io_control = self.io_control.get_mut()?;
        io_control.clear_line().await?;
        self.chat.request(msg.prompt);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<CtrlC> for StdioApp {
    async fn handle(&mut self, _: CtrlC, ctx: &mut Context<Self>) -> Result<()> {
        let io_control = self.io_control.get_mut()?;
        io_control.clear_line().await?;
        io_control.writeln("Closing the session 🙌").await?;
        self.substance.substance.interrupt()
    }
}

#[async_trait]
impl OnEvent<Result<String>> for StdioApp {
    async fn handle(&mut self, event: Result<String>, ctx: &mut Context<Self>) -> Result<()> {
        if self.asking {
            let io_control = self.io_control.get_mut()?;
            io_control.move_up().await?;
            io_control.clear_line().await?;
            self.asking = false;

            self.prompts.push_back(event?);
            if self.queue.is_empty() {
                if let Some(prompt) = self.prompts.pop_front() {
                    ctx.do_next(Next::do_async(ProcessingPrompt { prompt }));
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<SubEvent<Chat>> for StdioApp {
    async fn handle(&mut self, event: SubEvent<Chat>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                self.queue.add_message("Chat state has loaded");
                {
                    let state_ref = state.borrow();
                    for message in &state_ref.messages {}
                }
                self.state = Some(state);
            }
            SubEvent::Event(event) => match event {
                ChatEvent::Add { message } => {
                    let io_control = self.io_control.get_mut()?;
                    let role = match message.role {
                        Role::Request => "👤 Request:".blue(),
                        Role::Response => "🤖 Response:".yellow(),
                    };
                    io_control.writeln(&role.to_string()).await?;
                    io_control.write_md(&message.content).await?;
                }
                ChatEvent::SetThinking { flag } => {
                    if flag {
                        self.queue.add_message("Thinking...");
                        self.interval.start();
                    } else {
                        // TODO: Add an event to request...
                    }
                }
            },
            SubEvent::Lost => {
                self.state.take();
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<SubEvent<Live>> for StdioApp {
    async fn handle(&mut self, event: SubEvent<Live>, ctx: &mut Context<Self>) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                for message in state.borrow().messages.iter() {
                    self.queue.add_message(message);
                }
            }
            SubEvent::Event(event) => {
                let message = String::from(event);
                self.queue.add_message(&message);
            }
            SubEvent::Lost => {}
        }
        Ok(())
    }
}
