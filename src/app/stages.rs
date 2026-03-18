use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget};
use tui_checkbox::*;

#[derive(Debug, Clone)]
pub struct StageData {
    pub name: String,
    pub bit: usize,
    pub selected: bool,
    pub checked: bool,
}

impl StageData {
    pub fn new(name: &str, bit: usize, checked: bool) -> Self {
        StageData {
            name: name.to_owned(),
            bit,
            selected: false,
            checked,
        }
    }

    pub fn flip(&mut self) {
        self.checked = !self.checked
    }
}

#[derive(Debug)]
pub struct Stages {
    data: Vec<StageData>,
}

impl Stages {
    pub fn new(data: Vec<StageData>) -> Self {
        Self { data }
    }
}

impl Widget for Stages {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Stages");
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
