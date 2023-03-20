use anyhow::{bail, Result};
use std::{
    io::{self, Write}
};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     Terminal,
// };

mod app;
mod cmdbar;
mod strings;
mod keys;
mod command;

fn main() -> Result<()> {
    let mut termial = start_termina(io::stdout())?;

    Ok(())
}

fn run_app() {

}

fn start_termina<W: Write>(
    buf: W
)  {
    // let backend = CrosstermBackend::new(buf);
    // let mut termial = Terminal::new(backend)?;
    // termial.hide_cursor()?;
    // termial.clear()?;
    // Ok(termial)
}