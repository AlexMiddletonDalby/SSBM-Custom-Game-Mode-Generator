mod check_list;
mod code_generation;
mod cycle_button;
mod melee;

use check_list::CheckList;
use cycle_button::CycleButton;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use std::cmp;
use std::io;

use crate::app::cycle_button::CycleButtonData;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CursorDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Section {
    GameOptions,
    Stages,
    Items,
}

impl Section {
    fn direction(self) -> CursorDirection {
        match self {
            Self::GameOptions => CursorDirection::Horizontal,
            Self::Stages => CursorDirection::Vertical,
            Self::Items => CursorDirection::Vertical,
        }
    }
}

#[derive(Debug)]
pub struct App {
    stocks: CycleButtonData,
    item_frequency: CycleButtonData,
    stages: Vec<melee::Entry>,
    items: Vec<melee::Entry>,
    output_data: String,
    cursor_section: Section,
    cursor_pos: usize,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            stocks: CycleButtonData::with_states(vec![
                "Stocks: 1".to_string(),
                "Stocks: 2".to_string(),
                "Stocks: 3".to_string(),
                "Stocks: 4".to_string(),
            ]),
            item_frequency: CycleButtonData::with_states(vec![
                "Items: None".to_string(),
                "Items: Very Low".to_string(),
                "Items: Low".to_string(),
                "Items: Medium".to_string(),
                "Items: High".to_string(),
                "Items: Very High".to_string(),
            ]),

            stages: melee::default_stages(),
            items: melee::default_items(),
            output_data: String::new(),
            cursor_section: Section::GameOptions,
            cursor_pos: 0,
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

        let main_layout = Layout::vertical(vec![
            Constraint::Length(3),
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(block.inner(frame.area()));

        let game_options_block = Block::bordered()
            .title("Game options")
            .title_alignment(HorizontalAlignment::Left);

        let game_options = Layout::horizontal(vec![Constraint::Fill(1), Constraint::Fill(1)])
            .spacing(1)
            .split(game_options_block.inner(main_layout[0]));

        let stocks = CycleButton::new(self.stocks.clone());
        let item_frequency = CycleButton::new(self.item_frequency.clone());

        let stages_and_items =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_layout[1]);

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

        frame.render_widget(block, frame.area());
        frame.render_widget(game_options_block, main_layout[0]);
        frame.render_widget(stocks, game_options[0]);
        frame.render_widget(item_frequency, game_options[1]);
        frame.render_widget(stages, stages_and_items[0]);
        frame.render_widget(items, stages_and_items[1]);
        frame.render_widget(output, main_layout[2]);
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn current_section_rows(&self) -> usize {
        match self.cursor_section {
            Section::GameOptions => 2,
            Section::Stages => self.stages.len(),
            Section::Items => self.items.len(),
        }
    }

    fn update_selection(&mut self) {
        self.cursor_pos = cmp::min(self.cursor_pos, self.current_section_rows() - 1);

        self.stocks.selected = self.cursor_section == Section::GameOptions && self.cursor_pos == 0;

        self.item_frequency.selected =
            self.cursor_section == Section::GameOptions && self.cursor_pos == 1;

        for (index, stage) in self.stages.iter_mut().enumerate() {
            stage.checkbox.selected =
                self.cursor_section == Section::Stages && index == self.cursor_pos;
        }

        for (index, item) in self.items.iter_mut().enumerate() {
            item.checkbox.selected =
                self.cursor_section == Section::Items && index == self.cursor_pos;
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

    fn increment_cursor(&mut self) {
        if self.cursor_pos < self.current_section_rows() - 1 {
            self.cursor_pos += 1;
            self.update_selection();
        }
    }

    fn decrement_cursor(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.update_selection();
        }
    }

    fn handle_keys(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Up => match self.cursor_section.direction() {
                    CursorDirection::Vertical => {
                        if self.cursor_pos == 0 {
                            self.cursor_section = Section::GameOptions;
                            self.update_selection();
                        } else {
                            self.decrement_cursor();
                        }
                    }
                    CursorDirection::Horizontal => {}
                },
                KeyCode::Down => match self.cursor_section.direction() {
                    CursorDirection::Vertical => {
                        self.increment_cursor();
                    }
                    CursorDirection::Horizontal => {
                        self.cursor_section = Section::Stages;
                        self.update_selection();
                    }
                },
                KeyCode::Left => match self.cursor_section.direction() {
                    CursorDirection::Vertical => {
                        if self.cursor_section == Section::Items {
                            self.cursor_section = Section::Stages;
                            self.update_selection();
                        }
                    }
                    CursorDirection::Horizontal => {
                        self.decrement_cursor();
                    }
                },
                KeyCode::Right => match self.cursor_section.direction() {
                    CursorDirection::Vertical => {
                        if self.cursor_section == Section::Stages {
                            self.cursor_section = Section::Items;
                            self.update_selection();
                        }
                    }
                    CursorDirection::Horizontal => {
                        self.increment_cursor();
                    }
                },
                KeyCode::Char(' ') => {
                    match self.cursor_section {
                        Section::GameOptions => {
                            if self.cursor_pos == 0 {
                                self.stocks.next();
                            }
                            if self.cursor_pos == 1 {
                                self.item_frequency.next();
                            }
                        }
                        Section::Stages => {
                            self.stages[self.cursor_pos].checkbox.flip();
                        }
                        Section::Items => {
                            self.items[self.cursor_pos].checkbox.flip();
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
