mod code_generation;
mod stages;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use stages::{StageData, Stages};
use std::io;

#[derive(Debug)]
pub struct App {
    stage_data: Vec<StageData>,
    output_data: String,
    cursor_position: usize,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            stage_data: vec![
                StageData::new("Battlefield", 0, true),
                StageData::new("Big Blue", 1, false),
                StageData::new("Brinstar", 2, false),
                StageData::new("Brinstar Depths", 3, false),
                StageData::new("Corneria", 4, false),
                StageData::new("Dream Land N64", 5, true),
                StageData::new("Final Destination", 6, true),
                StageData::new("Flat Zone", 7, false),
                StageData::new("Fountain of Dreams", 8, true),
                StageData::new("Fourside", 9, false),
                StageData::new("Great Bay", 10, false),
                StageData::new("Green Greens", 11, false),
                StageData::new("Icicle Mountain", 12, false),
                StageData::new("Jungle Japes", 13, false),
                StageData::new("Kongo Jungle", 14, false),
                StageData::new("Kongo Jungle N64", 15, false),
                StageData::new("Mushroom Kingdom", 16, false),
                StageData::new("Mushroom Kingdom II", 17, false),
                StageData::new("Mute City", 18, false),
                StageData::new("Onett", 19, false),
                StageData::new("Poke Floats", 20, false),
                StageData::new("Pokemon Stadium", 21, true),
                StageData::new("Princess Peach's Castle", 22, false),
                StageData::new("Rainbow Cruise", 23, false),
                StageData::new("Temple", 24, false),
                StageData::new("Venom", 25, false),
                StageData::new("Yoshi's Island", 26, false),
                StageData::new("Yoshi's Island N64", 27, false),
                StageData::new("Yoshi's Story", 28, true),
            ],
            output_data: String::new(),
            cursor_position: 0,
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.update_stage_selection();
        self.update_output();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title("SSBM Custom Game Mode Generator v0.1")
            .title_alignment(HorizontalAlignment::Center)
            .padding(Padding::symmetric(2, 1));

        let stages = Stages::new(self.stage_data.clone());

        let output =
            Paragraph::new(self.output_data.clone()).block(Block::bordered().title("Output"));

        let layout =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .spacing(2)
                .split(block.inner(frame.area()));

        frame.render_widget(block, frame.area());
        frame.render_widget(stages, layout[0]);
        frame.render_widget(output, layout[1]);
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn update_stage_selection(&mut self) {
        for (index, stage) in self.stage_data.iter_mut().enumerate() {
            stage.selected = index == self.cursor_position;
        }
    }

    fn update_output(&mut self) {
        self.output_data = code_generation::generate(self.stage_data.clone());
    }

    fn handle_keys(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Up => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.update_stage_selection();
                    }
                }
                KeyCode::Down => {
                    if self.cursor_position < self.stage_data.len() - 1 {
                        self.cursor_position += 1;
                        self.update_stage_selection();
                    }
                }
                KeyCode::Char(' ') => {
                    self.stage_data[self.cursor_position].flip();
                    self.update_output();
                }
                KeyCode::Char('q') => self.quit(),
                _ => {}
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_keys(key_event),
            _ => {}
        };

        Ok(())
    }
}
