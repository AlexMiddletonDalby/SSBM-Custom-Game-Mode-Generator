use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};
use ratatui_textarea::TextArea;

#[derive(Debug)]
pub struct NumericButton<'a> {
    pub prefix: String,
    pub value: String,
    pub selected: bool,
    pub editing: bool,

    editor: TextArea<'a>,
}

impl<'a> NumericButton<'a> {
    pub fn new(prefix: &str, value: &str) -> Self {
        let mut s = Self {
            prefix: prefix.to_owned(),
            value: value.to_owned(),
            selected: false,
            editing: false,
            editor: TextArea::default(),
        };

        s.editor.set_placeholder_text("Enter a value");

        return s;
    }

    pub fn handle_key_press(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char(' ') => {
                self.editing = !self.editing;
                return true;
            }
            _ => return false,
        }
    }
}

impl<'a> Widget for &NumericButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.editing {
            self.editor.render(area, buf);
        } else {
            let p = Paragraph::new(self.prefix.clone() + &self.value).style(if self.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            });
            p.render(area, buf);
        }
    }
}
