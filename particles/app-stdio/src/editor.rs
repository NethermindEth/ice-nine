use anyhow::Result;
use rustyline::{
    error::ReadlineError,
    history::DefaultHistory,
    validate::{ValidationContext, ValidationResult, Validator},
    Cmd, Completer, Config, DefaultEditor, Editor, Event, Helper, Highlighter, Hinter, KeyCode,
    KeyEvent, Modifiers,
};
use tokio::io::{AsyncWriteExt, Stdout};

#[derive(Completer, Helper, Highlighter, Hinter)]
struct InputBlocker;

impl Validator for InputBlocker {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(if ctx.input().contains('\n') {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Valid(None)
        })
    }
}

pub struct IoControl {
    editor: Editor<InputBlocker, DefaultHistory>,
    stdout: Stdout,
}

impl IoControl {
    async fn write(&mut self, text: &str) -> Result<()> {
        self.stdout.write_all(text.as_ref()).await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn writeln(&mut self, text: &str) -> Result<()> {
        self.stdout.write_all(text.as_ref()).await?;
        self.stdout.write_all(b"\n").await?;
        self.stdout.flush().await?;
        Ok(())
    }

    async fn start_thinking(&mut self) -> Result<()> {
        self.editor.set_cursor_visibility(false)?;
        Ok(())
    }

    async fn stop_thinking(&mut self) -> Result<()> {
        self.editor.set_cursor_visibility(true)?;
        self.clear_line().await?;
        Ok(())
    }

    async fn clear_line(&mut self) -> Result<()> {
        self.stdout.write_all(b"\r").await?;
        self.stdout.flush().await?;
        Ok(())
    }
}
