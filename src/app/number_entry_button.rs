use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};
use ratatui_textarea::{Input, Key, TextArea};

#[derive(Debug)]
pub struct NumberEntryButton<'a> {
    pub prefix: String,
    pub value: u8,
    pub selected: bool,
    pub editing: bool,

    editor: TextArea<'a>,
}

fn is_numeric(key: KeyCode) -> bool {
    key == KeyCode::Char('0')
        || key == KeyCode::Char('1')
        || key == KeyCode::Char('2')
        || key == KeyCode::Char('3')
        || key == KeyCode::Char('4')
        || key == KeyCode::Char('5')
        || key == KeyCode::Char('6')
        || key == KeyCode::Char('7')
        || key == KeyCode::Char('8')
        || key == KeyCode::Char('9')
}

impl<'a> NumberEntryButton<'a> {
    pub fn new(prefix: &str, value: u8) -> Self {
        let mut s = Self {
            prefix: prefix.to_owned(),
            value,
            selected: false,
            editing: false,
            editor: TextArea::default(),
        };

        s.editor.set_placeholder_text("Enter a value");

        return s;
    }

    fn enter_edit_mode(&mut self) {
        self.editor.delete_line_by_head();
        self.editing = true;
    }

    fn exit_edit_mode(&mut self) {
        if let Some(line) = self.editor.lines().first().clone() {
            if let Ok(val) = line.parse::<u8>() {
                self.value = val;
            }
        }

        self.editing = false;
    }

    pub fn handle_key_press(&mut self, key: KeyCode) -> bool {
        if key == KeyCode::Enter || key == KeyCode::Char(' ') {
            if self.editing {
                self.exit_edit_mode();
            } else {
                self.enter_edit_mode();
            }
            return true;
        }
        if is_numeric(key) {
            self.editor.input(Input {
                key: Key::Char(key.as_char().unwrap()),
                ctrl: false,
                alt: false,
                shift: false,
            });
            return true;
        }

        return false;
    }
}

impl<'a> Widget for &NumberEntryButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.editing {
            self.editor.render(area, buf);
        } else {
            let p = Paragraph::new(self.prefix.clone() + &self.value.to_string()).style(
                if self.selected {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                },
            );
            p.render(area, buf);
        }
    }
}
