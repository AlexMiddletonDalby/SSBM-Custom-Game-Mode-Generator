use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub struct ActionButton {
    pub text: String,
    pub pressed_text: Option<String>,
    enabled: bool,
    selected: bool,
    has_been_pressed: bool,
}

impl<'a> ActionButton {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            pressed_text: None,
            enabled: true,
            selected: false,
            has_been_pressed: false,
        }
    }

    pub fn with_pressed_text(text: &str, pressed_text: &str) -> Self {
        Self {
            text: text.to_string(),
            pressed_text: Some(pressed_text.to_string()),
            enabled: true,
            selected: false,
            has_been_pressed: false,
        }
    }

    pub fn handle_key_press(&mut self, code: KeyCode, mut on_press: impl FnMut()) -> bool {
        if code == KeyCode::Char(' ') || code == KeyCode::Enter {
            on_press();
            self.has_been_pressed = true;
            return true;
        }

        return false;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !self.enabled() {
            self.selected = false;
            self.has_been_pressed = false;
        }
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        if !self.selected {
            self.has_been_pressed = false;
        }
    }

    fn get_text(&self) -> String {
        if let Some(text) = &self.pressed_text {
            if self.has_been_pressed {
                return text.clone();
            }
        }

        return self.text.clone();
    }
}

impl Widget for &ActionButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new(self.get_text())
            .style(if !self.enabled() {
                Style::default().add_modifier(Modifier::DIM)
            } else if self.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            })
            .alignment(HorizontalAlignment::Center)
            .block(Block::bordered().border_style(Style::default().dark_gray()));
        p.render(area, buf);
    }
}
