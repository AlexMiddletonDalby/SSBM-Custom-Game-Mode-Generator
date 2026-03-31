use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget};
use tui_checkbox::*;

#[derive(Debug, Clone)]
pub struct CheckboxEntry {
    pub name: String,
    pub selected: bool,
    pub checked: bool,
}

impl CheckboxEntry {
    pub fn new(name: &str, checked: bool) -> Self {
        CheckboxEntry {
            name: name.to_owned(),
            selected: false,
            checked,
        }
    }

    pub fn flip(&mut self) {
        self.checked = !self.checked
    }
}

#[derive(Debug)]
pub struct CheckList {
    pub title: String,
    pub entries: Vec<CheckboxEntry>,
}

impl CheckList {
    pub fn new(title: &str, entries: Vec<CheckboxEntry>) -> Self {
        Self {
            title: title.to_owned(),
            entries,
        }
    }

    pub fn handle_key_press(&mut self, key: KeyCode, cursor_pos: usize) -> bool {
        match key {
            KeyCode::Char(' ') => {
                self.entries[cursor_pos].flip();
                return true;
            }
            _ => return false,
        }
    }
}

impl Widget for &CheckList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(self.title.clone())
            .border_style(Style::new().add_modifier(Modifier::ITALIC));
        let widgets: Vec<Checkbox> = self
            .entries
            .iter()
            .map(|entry| {
                Checkbox::new(entry.name.clone(), entry.checked).style(if entry.selected {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                })
            })
            .collect();

        let layout = Layout::vertical(self.entries.iter().map(|_entry| Constraint::Length(1)))
            .split(block.inner(area));

        for (index, widget) in widgets.iter().enumerate() {
            widget.render(layout[index], buf);
        }

        block.render(area, buf);
    }
}
