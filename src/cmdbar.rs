use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    text::{Span, Spans}
};

enum DrawListEntry {
    LineBreak,
    Splitter,
    Command(Command),
}

struct Command {
    txt: String,
    enabled: bool,
    line: usize,
}

pub struct CommandBar {
    draw_list: Vec<DrawListEntry>,
}

impl CommandBar {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        
    }
}