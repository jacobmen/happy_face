use crate::state::{State};
mod util;

use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::Write;
use crossterm::terminal::{self};
use crossterm::{ExecutableCommand};

pub struct Renderer<W: Write> {
    terminal: Terminal<CrosstermBackend<W>>,
}

impl<W: Write> Renderer<W> {
    pub fn new(mut out: W) -> Result<Renderer<W>> {
        terminal::enable_raw_mode()?;
        out.execute(terminal::EnterAlternateScreen)?;

        Ok(Renderer { terminal: Terminal::new(CrosstermBackend::new(out))? })
    }

    pub fn render(&mut self, state: &State) -> Result<()> {
        self.terminal.draw(|frame| ui::draw(frame, state, frame.size()))?;
        Ok(())
    }
}
