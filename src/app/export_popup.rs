use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui_textarea::TextArea;

use crate::app::Section;

#[derive(Debug)]
struct LabelledTextArea<'a> {
    label: String,
    placeholder: String,
    value: String,
    selected: bool,
    editing: bool,
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
            style = style.add_modifier(Modifier::ITALIC);
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

#[derive(Debug)]
pub struct ExportPopup<'a> {
    name: LabelledTextArea<'a>,
    description: LabelledTextArea<'a>,
    author: LabelledTextArea<'a>,
}

impl<'a> ExportPopup<'a> {
    pub fn new() -> Self {
        Self {
            name: LabelledTextArea::new("Name: ", "My cool gamemode"),
            description: LabelledTextArea::new(
                "Description: ",
                "Fox only, no items, Final Destination",
            ),
            author: LabelledTextArea::new("Author: ", "John Melee"),
        }
    }

    pub fn is_editing_text(&self) -> bool {
        self.name.editing || self.description.editing || self.author.editing
    }

    pub fn update_selection(&mut self, cursor_pos: usize, cursor_section: Section) {
        self.name.selected = cursor_pos == 0 && cursor_section == Section::ExportOptions;
        self.description.selected = cursor_pos == 1 && cursor_section == Section::ExportOptions;
        self.author.selected = cursor_pos == 2 && cursor_section == Section::ExportOptions;
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) -> bool {
        if self.name.selected {
            return self.name.handle_key_press(event);
        }
        if self.description.selected {
            return self.description.handle_key_press(event);
        }
        if self.author.selected {
            return self.author.handle_key_press(event);
        }

        return false;
    }
}

impl<'a> Widget for &ExportPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::bordered()
            .title("Code Options")
            .title_alignment(Alignment::Center)
            .border_style(Style::new().add_modifier(Modifier::ITALIC))
            .padding(Padding::symmetric(1, 1));

        let layout = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(popup_block.inner(area));

        popup_block.render(area, buf);
        self.name.render(layout[0], buf);
        self.description.render(layout[1], buf);
        self.author.render(layout[2], buf);
    }
}
