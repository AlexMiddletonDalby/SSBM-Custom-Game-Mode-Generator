use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui_textarea::TextArea;

#[derive(Debug)]
pub struct LabelledTextArea<'a> {
    label: String,
    placeholder: String,
    pub value: String,
    pub selected: bool,
    pub editing: bool,
    editor: TextArea<'a>,
}

impl<'a> LabelledTextArea<'a> {
    pub fn new(text: &str, placeholder_text: &str) -> Self {
        Self {
            label: text.to_string(),
            placeholder: placeholder_text.to_owned(),
            value: String::new(),
            selected: false,
            editing: false,
            editor: TextArea::default(),
        }
    }

    fn enter_edit_mode(&mut self) {
        self.editing = true;
    }

    fn exit_edit_mode(&mut self, commit: bool) {
        if commit {
            if let Some(val) = self.editor.lines().first().clone() {
                self.value = val.clone();
            }
        }

        self.editing = false;
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) -> bool {
        let key = event.code;

        if self.editing {
            if key == KeyCode::Enter {
                self.exit_edit_mode(true);
            } else if key == KeyCode::Esc {
                self.exit_edit_mode(false);
            } else {
                self.editor.input(event);
            }

            return true;
        } else {
            if key == KeyCode::Char(' ') || key == KeyCode::Enter {
                self.enter_edit_mode();
                return true;
            }
        }

        return false;
    }

    pub fn render_value_text(&self, area: Rect, buf: &mut Buffer) {
        let mut style = if self.selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        if self.value.is_empty() {
            style = style
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::DIM);
        }

        let value_text = Paragraph::new(if self.value.is_empty() {
            self.placeholder.clone()
        } else {
            self.value.clone()
        })
        .style(style);

        value_text.render(area, buf);
    }

    pub fn render_editor(&self, area: Rect, buf: &mut Buffer) {
        self.editor.render(area, buf);
    }
}

impl<'a> Widget for &LabelledTextArea<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = Paragraph::new(self.label.clone());

        let layout =
            Layout::horizontal(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(area);

        label.render(layout[0], buf);
        if self.editing {
            self.render_editor(layout[1], buf);
        } else {
            self.render_value_text(layout[1], buf);
        }
    }
}
