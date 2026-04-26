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
    pub enabled: bool,
}

impl CheckList {
    pub fn new(title: &str, entries: Vec<CheckboxEntry>) -> Self {
        Self {
            title: title.to_owned(),
            entries,
            enabled: true,
        }
    }

    pub fn handle_key_press(&mut self, key: KeyCode, cursor_pos: usize) -> bool {
        if key == KeyCode::Char(' ') || key == KeyCode::Enter {
            self.entries[cursor_pos].flip();
            return true;
        }
        if key == KeyCode::Char('a') {
            let all_checked = self.entries.iter().all(|entry| entry.checked);
            for entry in &mut self.entries {
                entry.checked = !all_checked
            }
            return true;
        }

        return false;
    }
}

impl Widget for &CheckList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(self.title.clone())
            .style(if self.enabled {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            })
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
