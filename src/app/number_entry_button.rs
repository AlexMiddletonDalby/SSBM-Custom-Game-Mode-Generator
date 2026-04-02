use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};
use ratatui_textarea::{Input, Key, TextArea};

#[derive(Debug)]
pub struct NumberEntryButton<'a> {
    pub prefix: String,
    pub value: u8,
    pub suffix: String,
    pub selected: bool,
    pub editing: bool,
    pub zero_text: Option<String>,

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
    pub fn new(prefix: &str, value: u8, suffix: &str, zero_text: Option<String>) -> Self {
        let mut s = Self {
            prefix: prefix.to_owned(),
            value,
            suffix: suffix.to_owned(),
            selected: false,
            editing: false,
            editor: TextArea::default(),
            zero_text,
        };

        s.editor.set_placeholder_text("Enter a value");

        return s;
    }

    fn enter_edit_mode(&mut self) {
        self.editor.delete_line_by_head();
        self.editing = true;
    }

    fn exit_edit_mode(&mut self, commit: bool) {
        if commit {
            if let Some(line) = self.editor.lines().first().clone() {
                if let Ok(val) = line.parse::<u8>() {
                    if val != 0 || self.zero_text.is_some() {
                        self.value = val;
                    }
                }
            }
        }

        self.editing = false;
    }

    fn editor_has_space(&self) -> bool {
        if let Some(line) = self.editor.lines().first().clone() {
            return line.chars().count() < 2;
        }

        return true;
    }

    pub fn handle_key_press(&mut self, key: KeyCode) -> bool {
        if key == KeyCode::Enter || key == KeyCode::Char(' ') {
            if self.editing {
                self.exit_edit_mode(true);
            } else {
                self.enter_edit_mode();
            }
            return true;
        }
        if self.editing {
            if is_numeric(key) && self.editor_has_space() {
                self.editor.input(Input {
                    key: Key::Char(key.as_char().unwrap()),
                    ctrl: false,
                    alt: false,
                    shift: false,
                });
            }
            if key == KeyCode::Backspace {
                self.editor.input(Input {
                    key: Key::Backspace,
                    ctrl: false,
                    alt: false,
                    shift: false,
                });
            }

            if key == KeyCode::Esc {
                self.exit_edit_mode(false);
            }

            return true;
        }

        return false;
    }

    fn render_editor(&self, area: Rect, buf: &mut Buffer) {
        self.editor.render(area, buf);
    }

    fn render_label(&self, area: Rect, buf: &mut Buffer) {
        let mut text = self.prefix.clone() + &self.value.to_string() + &self.suffix;
        if let Some(zero_text) = self.zero_text.clone()
            && self.value == 0
        {
            text = self.prefix.clone() + &zero_text;
        }

        let p = Paragraph::new(text).style(if self.selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
        p.render(area, buf);
    }
}

impl<'a> Widget for &NumberEntryButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.editing {
            self.render_editor(area, buf);
        } else {
            self.render_label(area, buf);
        }
    }
}
