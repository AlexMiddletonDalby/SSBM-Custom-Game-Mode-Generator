use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};

#[derive(Debug, Clone)]
pub struct CycleButtonData {
    pub selected: bool,
    pub current_state: usize,
    pub states: Vec<String>,
}

impl CycleButtonData {
    pub fn with_states(states: Vec<String>) -> Self {
        Self {
            selected: false,
            current_state: 0,
            states,
        }
    }

    pub fn next(&mut self) {
        self.current_state = self.current_state + 1;
        if self.current_state >= self.states.len() {
            self.current_state = 0;
        }
    }
}

#[derive(Debug)]
pub struct CycleButton {
    data: CycleButtonData,
}

impl CycleButton {
    pub fn new(data: CycleButtonData) -> Self {
        Self { data }
    }
}

impl Widget for CycleButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new(self.data.states[self.data.current_state].clone()).style(
            if self.data.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            },
        );
        p.render(area, buf);
    }
}
