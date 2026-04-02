use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub struct ActionButton {
    pub selected: bool,
    pub text: String,
}

impl ActionButton {
    pub fn new(text: &str) -> Self {
        Self {
            selected: false,
            text: text.to_string(),
        }
    }
}

impl Widget for &ActionButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new(self.text.clone())
            .style(if self.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            })
            .alignment(HorizontalAlignment::Center)
            .block(Block::bordered().border_style(Style::default().dark_gray()));
        p.render(area, buf);
    }
}
