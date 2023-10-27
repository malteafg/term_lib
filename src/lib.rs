use crossterm::{cursor, event, execute, style, terminal};
use futures::{future::Fuse, stream::Next, FutureExt, StreamExt};
use thiserror::Error;

pub mod command;
pub mod display;
pub mod input;
pub mod prompt;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("yaml serialization error")]
    SerdeError(#[from] serde_yaml::Error),
    #[error("external cmd error")]
    ExternalCmdError,

    #[error("parsing error")]
    ParsingError(#[from] std::num::ParseIntError),

    #[error("escape was pressed")]
    Escape,
    #[error("signal to quit program was sent")]
    Quit,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn init<W: std::io::Write>(w: &mut W) -> Result<()> {
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn quit<W: std::io::Write>(w: &mut W) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    Ok(())
}

pub struct AsyncEventHandler {
    event_stream: crossterm::event::EventStream,
}

impl AsyncEventHandler {
    pub fn new() -> Self {
        Self {
            event_stream: crossterm::event::EventStream::new(),
        }
    }

    pub fn next(&mut self) -> Fuse<Next<'_, event::EventStream>> {
        self.event_stream.next().fuse()
    }
}
