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
    title: String,
    data: Vec<CheckboxEntry>,
}

impl CheckList {
    pub fn new(title: &str, data: Vec<CheckboxEntry>) -> Self {
        Self {
            title: title.to_owned(),
            data,
        }
    }
}

impl Widget for CheckList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(self.title);
        let widgets: Vec<Checkbox> = self
            .data
            .iter()
            .map(|entry| {
                Checkbox::new(entry.name.clone(), entry.checked).style(if entry.selected {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                })
            })
            .collect();

        let layout = Layout::vertical(self.data.iter().map(|_entry| Constraint::Length(1)))
            .split(block.inner(area));

        for (index, widget) in widgets.iter().enumerate() {
            widget.render(layout[index], buf);
        }

        block.render(area, buf);
    }
}
