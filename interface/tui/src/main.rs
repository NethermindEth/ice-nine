use anyhow::Result;
use async_trait::async_trait;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use crb::agent::{Agent, AgentSession, DoAsync, Next, Runnable};

#[tokio::main]
async fn main() -> Result<()> {
    TuiApp::new().run().await?;

    /*
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
    */

    Ok(())
}

struct TuiApp {
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Agent for TuiApp {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Configure)
    }
}

struct Configure;

#[async_trait]
impl DoAsync<Configure> for TuiApp {
    async fn once(&mut self, _: &mut Configure) -> Result<Next<Self>> {
        ratatui::try_init()?;
        Ok(Next::events())
    }
}

/*
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
*/
