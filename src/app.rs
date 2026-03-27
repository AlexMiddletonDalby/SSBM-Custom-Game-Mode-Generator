mod check_list;
mod code_generation;
mod cycle_button;
mod melee;
mod number_entry_button;

use check_list::CheckList;
use cycle_button::CycleButton;
use number_entry_button::NumberEntryButton;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use std::cmp;
use std::io;

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
struct Widgets<'a> {
    mode: CycleButton,
    stocks: NumberEntryButton<'a>,
    time: NumberEntryButton<'a>,
    item_frequency: CycleButton,
    stages: CheckList,
    items: CheckList,
}

#[derive(Debug)]
pub struct App<'a> {
    output_data: String,
    cursor_section: Section,
    cursor_pos: usize,
    exit: bool,

    widgets: Widgets<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            output_data: String::new(),
            cursor_section: Section::GameOptions,
            cursor_pos: 0,
            exit: false,
            widgets: Widgets {
                mode: CycleButton::with_states(vec![
                    "Mode: Direct".to_string(),
                    "Mode: Doubles".to_string(),
                ]),
                stocks: NumberEntryButton::new("Stocks: ", 4, ""),
                time: NumberEntryButton::new("Time: ", 8, " minutes"),
                item_frequency: CycleButton::with_states(vec![
                    "Items: None".to_string(),
                    "Items: Very Low".to_string(),
                    "Items: Low".to_string(),
                    "Items: Medium".to_string(),
                    "Items: High".to_string(),
                    "Items: Very High".to_string(),
                ]),
                stages: CheckList::new(
                    "Stages",
                    melee::default_stages()
                        .iter()
                        .map(|stage| stage.checkbox.clone())
                        .collect(),
                ),
                items: CheckList::new(
                    "Items",
                    melee::default_items()
                        .iter()
                        .map(|stage| stage.checkbox.clone())
                        .collect(),
                ),
            },
        }
    }
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.update_selection();
        self.update_output();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
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

        let game_options = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .split(game_options_block.inner(main_layout[0]));

        let stages_and_items =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_layout[1]);

        let output =
            Paragraph::new(self.output_data.clone()).block(Block::bordered().title("Output"));

        frame.render_widget(block, frame.area());
        frame.render_widget(game_options_block, main_layout[0]);
        frame.render_widget(&self.widgets.mode, game_options[0]);
        frame.render_widget(&self.widgets.stocks, game_options[1]);
        frame.render_widget(&self.widgets.time, game_options[2]);
        frame.render_widget(&self.widgets.item_frequency, game_options[3]);
        frame.render_widget(&self.widgets.stages, stages_and_items[0]);
        frame.render_widget(&self.widgets.items, stages_and_items[1]);
        frame.render_widget(output, main_layout[2]);
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn current_section_rows(&self) -> usize {
        match self.cursor_section {
            Section::GameOptions => 4,
            Section::Stages => self.widgets.stages.entries.len(),
            Section::Items => self.widgets.items.entries.len(),
        }
    }

    fn update_selection(&mut self) {
        self.cursor_pos = cmp::min(self.cursor_pos, self.current_section_rows() - 1);

        self.widgets.mode.selected =
            self.cursor_section == Section::GameOptions && self.cursor_pos == 0;

        self.widgets.stocks.selected =
            self.cursor_section == Section::GameOptions && self.cursor_pos == 1;

        self.widgets.time.selected =
            self.cursor_section == Section::GameOptions && self.cursor_pos == 2;

        self.widgets.item_frequency.selected =
            self.cursor_section == Section::GameOptions && self.cursor_pos == 3;

        for (index, entry) in self.widgets.stages.entries.iter_mut().enumerate() {
            entry.selected = self.cursor_section == Section::Stages && index == self.cursor_pos;
        }

        for (index, entry) in self.widgets.items.entries.iter_mut().enumerate() {
            entry.selected = self.cursor_section == Section::Items && index == self.cursor_pos;
        }
    }

    fn update_output(&mut self) {
        self.output_data = code_generation::generate(
            self.widgets
                .stages
                .entries
                .iter()
                .enumerate()
                .map(|(index, entry)| code_generation::Bit {
                    pos: melee::default_stages()[index].bit,
                    state: entry.checked,
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
                KeyCode::Char('q') => self.quit(),
                key => {
                    let mut handled: bool = false;
                    match self.cursor_section {
                        Section::GameOptions => match self.cursor_pos {
                            0 => handled = self.widgets.mode.handle_key_press(key),
                            1 => handled = self.widgets.stocks.handle_key_press(key),
                            2 => handled = self.widgets.time.handle_key_press(key),
                            3 => handled = self.widgets.item_frequency.handle_key_press(key),
                            _ => {}
                        },
                        Section::Stages => {
                            handled = self.widgets.stages.handle_key_press(key, self.cursor_pos);
                        }
                        Section::Items => {
                            handled = self.widgets.items.handle_key_press(key, self.cursor_pos);
                        }
                    }
                    if handled {
                        self.update_output();
                    }
                }
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
