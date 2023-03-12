use std::cell::RefCell;
use anyhow::{Result};
use tui::{backend::Backend, Frame, layout::{Direction, Layout, Constraint}};

use crate::cmdbar::CommandBar;

pub struct App {
    cmdbar: RefCell<CommandBar>,
}

impl App {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let fsize = f.size();
        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(2),
                Constraint::Length(self.cmdbar.borrow().height()),
            ].as_ref(),)
            .split(fsize);
        self.cmdbar.borrow().draw(f, chunks_main[2]);
        Ok(())
    }
}