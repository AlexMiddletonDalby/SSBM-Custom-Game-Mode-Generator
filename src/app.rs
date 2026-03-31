mod check_list;
mod code_generation;
mod cycle_button;
mod export_options;
mod melee;
mod number_entry_button;

use check_list::CheckList;
use cycle_button::CycleButton;
use export_options::ExportOptionsPopup;
use number_entry_button::NumberEntryButton;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Clear;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use std::cmp;
use std::io;

use crate::app::code_generation::GameMode;

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
    export_options: ExportOptionsPopup<'a>,
}

#[derive(Debug)]
pub struct App<'a> {
    output_data: String,
    cursor_section: Section,
    cursor_pos: usize,
    showing_export_popup: bool,
    exit: bool,

    widgets: Widgets<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            output_data: String::new(),
            cursor_section: Section::GameOptions,
            cursor_pos: 0,
            showing_export_popup: false,
            exit: false,
            widgets: Widgets {
                mode: CycleButton::with_states(vec![
                    "Mode: Direct".to_string(),
                    "Mode: Teams".to_string(),
                ]),
                stocks: NumberEntryButton::new("Stocks: ", 4, "", None),
                time: NumberEntryButton::new("Time: ", 8, " minutes", Some("None".to_string())),
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
                        .map(|item| item.checkbox.clone())
                        .collect(),
                ),
                export_options: ExportOptionsPopup::new(),
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
            .border_style(Style::new().add_modifier(Modifier::ITALIC))
            .padding(Padding::symmetric(1, 0))
            .style(if self.showing_export_popup {
                Style::default().add_modifier(Modifier::DIM)
            } else {
                Style::default()
            });

        let main_layout = Layout::vertical(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(block.inner(frame.area()));

        let game_options_block = Block::bordered()
            .title("Game options")
            .border_style(Style::new().add_modifier(Modifier::ITALIC))
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

        let generate_button = Paragraph::new("< Generate Code >")
            .alignment(HorizontalAlignment::Center)
            .block(Block::bordered().border_style(Style::default().dark_gray()));

        let action_buttons = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Length(21),
            Constraint::Fill(1),
        ])
        .split(main_layout[2]);

        frame.render_widget(block, frame.area());
        frame.render_widget(game_options_block, main_layout[0]);
        frame.render_widget(&self.widgets.mode, game_options[0]);
        frame.render_widget(&self.widgets.stocks, game_options[1]);
        frame.render_widget(&self.widgets.time, game_options[2]);
        frame.render_widget(&self.widgets.item_frequency, game_options[3]);
        frame.render_widget(&self.widgets.stages, stages_and_items[0]);
        frame.render_widget(&self.widgets.items, stages_and_items[1]);
        frame.render_widget(generate_button, action_buttons[1]);

        if self.showing_export_popup {
            let popup = frame
                .area()
                .centered(Constraint::Percentage(80), Constraint::Length(10));

            frame.render_widget(Clear, popup);
            frame.render_widget(&self.widgets.export_options, popup);
        }
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
        let mode = match self.widgets.mode.current_state {
            1 => GameMode::Doubles,
            _ => GameMode::Direct,
        };

        let stocks = self.widgets.stocks.value;

        let time_limit: Option<u8> = match self.widgets.time.value {
            0 => None,
            limit => Some(limit),
        };

        let stages: Vec<code_generation::Bit> = self
            .widgets
            .stages
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| code_generation::Bit {
                pos: melee::default_stages()[index].bit,
                state: entry.checked,
            })
            .collect();

        let item_frequency = match self.widgets.item_frequency.current_state {
            1 => code_generation::ItemFrequency::VeryLow,
            2 => code_generation::ItemFrequency::Low,
            3 => code_generation::ItemFrequency::Medium,
            4 => code_generation::ItemFrequency::High,
            5 => code_generation::ItemFrequency::VeryHigh,
            _ => code_generation::ItemFrequency::None,
        };

        let items: Vec<code_generation::Bit> = self
            .widgets
            .items
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| code_generation::Bit {
                pos: melee::default_items()[index].bit,
                state: entry.checked,
            })
            .collect();

        self.output_data =
            code_generation::generate(mode, stocks, time_limit, stages, item_frequency, items);
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
                KeyCode::Char('p') => self.showing_export_popup = !self.showing_export_popup,
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
