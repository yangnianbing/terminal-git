use std::borrow::Cow;
use crate::{
    strings,
    keys::SharedKeyConfig,
    command::CommandInfo,
};

use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    text::{Span, Spans},
    Frame, widgets::Paragraph
};

enum DrawListEntry {
    LineBreak,
    Splitter,
    Command(Command),
}

struct Command {
    txt: String,
}

pub struct CommandBar {
    draw_list: Vec<DrawListEntry>,
    cmd_infos: Vec<Command>,
    key_config: SharedKeyConfig,
}

impl CommandBar {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let splitter = Span::raw(Cow::from(strings::cmd_splitter(&self.key_config)));

        let texts = self.draw_list
            .split(|c| matches!(c, DrawListEntry::LineBreak
            ))
            .map(|c_arr| {
                Spans::from(
                    c_arr.iter()
                        .map(|c| match c {
                            DrawListEntry::Command(c) => {
                                Span::raw(c.txt.as_str())
                            }
                            DrawListEntry::LineBreak => {
                                Span::raw("")
                            }
                            DrawListEntry::Splitter => {
                                splitter.clone()
                            }
                        }).collect::<Vec<Span>>()
                )
            }).collect::<Vec<Spans>>();

        f.render_widget(Paragraph::new(texts).alignment(Alignment::Left), r);
    }

    pub fn set_cmds(&mut self, cmds: Vec<Command>) {
        self.cmd_infos = cmds;
        self.refresh_list();
    }

    pub fn refresh_list(&mut self) {
        self.draw_list.clear();

        for c in &self.cmd_infos {
           self.draw_list.push(
                DrawListEntry::Command(Command { 
                    txt: c.txt.to_string(), 
                })
            ) 
        }
    }

    pub const fn height(&self) -> u16 {
        1_u16
    }
}