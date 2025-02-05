use anyhow::{anyhow, Result};
use colored::Colorize;
use crb::core::time::Instant;
use derive_more::{Deref, DerefMut};
use rustyline::{
    error::ReadlineError,
    history::DefaultHistory,
    validate::{ValidationContext, ValidationResult, Validator},
    Cmd, Completer, Config, DefaultEditor, Editor, Event, Helper, Highlighter, Hinter, KeyCode,
    KeyEvent, Modifiers,
};
use tokio::io::{self, AsyncWriteExt, Stdout};

pub static RATE: u64 = 200;

#[derive(Completer, Helper, Highlighter, Hinter)]
pub struct InputBlocker;

impl Validator for InputBlocker {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        let result = {
            if ctx.input().contains('\n') {
                ValidationResult::Invalid(None)
            } else {
                ValidationResult::Valid(None)
            }
        };
        Ok(result)
    }
}

#[derive(Deref, DerefMut)]
pub struct IoControl {
    #[deref]
    #[deref_mut]
    editor: Editor<InputBlocker, DefaultHistory>,
    stdout: Stdout,
    started: Instant,
    rate: u64,
    spinner: Box<[char]>,
}

impl IoControl {
    pub fn new() -> Result<Self> {
        let mut editor = Editor::new()?;
        editor.set_helper(Some(InputBlocker));
        Ok(Self {
            editor,
            stdout: io::stdout(),
            started: Instant::now(),
            rate: RATE,
            spinner: Box::new(['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾']),
        })
    }

    /*
    pub fn prompt(&mut self) -> Result<String> {
        let readline = self.readline(">> ");
        match readline {
            Ok(line) => {
                return Ok(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
        Err(anyhow!("readline interrupted"))
    }
    */

    pub async fn write(&mut self, text: &str) -> Result<()> {
        self.stdout.write_all(text.as_ref()).await?;
        self.stdout.flush().await?;
        Ok(())
    }

    pub async fn writeln(&mut self, text: &str) -> Result<()> {
        self.stdout.write_all(text.as_ref()).await?;
        self.stdout.write_all(b"\n").await?;
        self.stdout.flush().await?;
        Ok(())
    }

    pub async fn write_md(&mut self, text: &str) -> Result<()> {
        let render = termimad::text(text).to_string();
        self.writeln(&render).await
    }

    pub async fn clear_line(&mut self) -> Result<()> {
        self.stdout.write_all(b"\r\x1B[2K").await?;
        self.stdout.flush().await?;
        Ok(())
    }

    pub async fn render_progress(&mut self, reason: &str) -> Result<()> {
        let elapsed = self.started.elapsed().as_millis() as u64;
        let len = self.spinner.len() as u64;
        let idx = elapsed / self.rate % len;
        let mut status = String::new();
        let current_char = self.spinner[idx as usize];
        status.push_str(&current_char.to_string().green().to_string());
        status.push_str(" ");
        status.push_str(&reason);
        self.clear_line().await?;
        self.write(&status).await?;
        Ok(())
    }
}
