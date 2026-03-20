mod check_list;
mod code_generation;
mod melee;

use check_list::CheckList;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use std::cmp;
use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Section {
    Stages,
    Items,
}

#[derive(Debug)]
pub struct App {
    stages: Vec<melee::Entry>,
    items: Vec<melee::Entry>,
    output_data: String,
    cursor_section: Section,
    cursor_row: usize,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            stages: melee::default_stages(),
            items: melee::default_items(),
            output_data: String::new(),
            cursor_section: Section::Stages,
            cursor_row: 0,
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.update_selection();
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

        let stages = CheckList::new(
            "Stages",
            self.stages
                .iter()
                .map(|stage| stage.checkbox.clone())
                .collect(),
        );
        let items = CheckList::new(
            "Items",
            self.items
                .iter()
                .map(|item| item.checkbox.clone())
                .collect(),
        );

        let output =
            Paragraph::new(self.output_data.clone()).block(Block::bordered().title("Output"));

        let main_layout =
            Layout::vertical(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(block.inner(frame.area()));
        let options_layout =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_layout[0]);

        frame.render_widget(block, frame.area());
        frame.render_widget(stages, options_layout[0]);
        frame.render_widget(items, options_layout[1]);
        frame.render_widget(output, main_layout[1]);
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn current_section_rows(&self) -> usize {
        match self.cursor_section {
            Section::Stages => self.stages.len(),
            Section::Items => self.items.len(),
        }
    }

    fn update_selection(&mut self) {
        self.cursor_row = cmp::min(self.cursor_row, self.current_section_rows() - 1);

        for (index, stage) in self.stages.iter_mut().enumerate() {
            stage.checkbox.selected =
                self.cursor_section == Section::Stages && index == self.cursor_row;
        }

        for (index, item) in self.items.iter_mut().enumerate() {
            item.checkbox.selected =
                self.cursor_section == Section::Items && index == self.cursor_row;
        }
    }

    fn update_output(&mut self) {
        self.output_data = code_generation::generate(
            self.stages
                .iter()
                .map(|stage| code_generation::Bit {
                    pos: stage.bit,
                    state: stage.checkbox.checked,
                })
                .collect(),
        );
    }

    fn handle_keys(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Up => {
                    if self.cursor_row > 0 {
                        self.cursor_row -= 1;
                        self.update_selection();
                    }
                }
                KeyCode::Down => {
                    if self.cursor_row < self.current_section_rows() - 1 {
                        self.cursor_row += 1;
                        self.update_selection();
                    }
                }
                KeyCode::Left => {
                    if self.cursor_section == Section::Items {
                        self.cursor_section = Section::Stages;
                        self.update_selection();
                    }
                }
                KeyCode::Right => {
                    if self.cursor_section == Section::Stages {
                        self.cursor_section = Section::Items;
                        self.update_selection();
                    }
                }
                KeyCode::Char(' ') => {
                    match self.cursor_section {
                        Section::Stages => {
                            self.stages[self.cursor_row].checkbox.flip();
                        }
                        Section::Items => {
                            self.items[self.cursor_row].checkbox.flip();
                        }
                    }
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
