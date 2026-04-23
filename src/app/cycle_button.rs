use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};

#[derive(Debug)]
pub struct CycleButton {
    pub selected: bool,
    pub current_state: usize,
    states: Vec<String>,
}

impl CycleButton {
    pub fn with_states(states: Vec<String>) -> Self {
        Self {
            selected: false,
            current_state: 0,
            states,
        }
    }

    pub fn handle_key_press(&mut self, code: KeyCode) -> bool {
        if code == KeyCode::Char(' ') || code == KeyCode::Enter {
            self.current_state = self.current_state + 1;
            if self.current_state >= self.states.len() {
                self.current_state = 0;
            }
            return true;
        }

        return false;
    }
}

impl Widget for &CycleButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new(self.states[self.current_state].clone()).style(if self.selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
        p.render(area, buf);
    }
}
