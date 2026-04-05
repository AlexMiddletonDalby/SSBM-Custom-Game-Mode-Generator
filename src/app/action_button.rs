use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub struct ActionButton {
    pub selected: bool,
    pub text: String,
}

impl<'a> ActionButton {
    pub fn new(text: &str) -> Self {
        Self {
            selected: false,
            text: text.to_string(),
        }
    }

    pub fn handle_key_press(&mut self, code: KeyCode, mut on_press: impl FnMut()) -> bool {
        if code == KeyCode::Char(' ') || code == KeyCode::Enter {
            on_press();
            return true;
        }

        return false;
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
