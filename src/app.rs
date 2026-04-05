mod action_button;
mod check_list;
mod code_generation;
mod cycle_button;
mod labelled_text_area;
mod melee;
mod number_entry_button;

use action_button::ActionButton;
use check_list::CheckList;
use cycle_button::CycleButton;
use labelled_text_area::LabelledTextArea;
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
pub enum Section {
    GameOptions,
    Stages,
    Items,
    Footer,
    ExportOptions,
    ExportFooter,
    ResultsScreenFooter,
}

impl Section {
    fn direction(&self) -> CursorDirection {
        match self {
            Self::GameOptions => CursorDirection::Horizontal,
            Self::Stages => CursorDirection::Vertical,
            Self::Items => CursorDirection::Vertical,
            Self::Footer => CursorDirection::Horizontal,
            Self::ExportOptions => CursorDirection::Vertical,
            Self::ExportFooter => CursorDirection::Horizontal,
            Self::ResultsScreenFooter => CursorDirection::Horizontal,
        }
    }
}

#[derive(Debug)]
struct Cursor {
    section: Section,
    pos: usize,
}

impl Cursor {
    fn is_on(&self, section: Section, pos: usize) -> bool {
        section == self.section && pos == self.pos
    }
}

#[derive(Debug)]
struct ExportPopup<'a> {
    name: LabelledTextArea<'a>,
    description: LabelledTextArea<'a>,
    author: LabelledTextArea<'a>,
    back: ActionButton,
    confirm: ActionButton,
}

#[derive(Debug)]
struct MainView<'a> {
    mode: CycleButton,
    stocks: NumberEntryButton<'a>,
    time: NumberEntryButton<'a>,
    item_frequency: CycleButton,
    stages: CheckList,
    items: CheckList,
    generate: ActionButton,
    export_popup: ExportPopup<'a>,
}

#[derive(Debug)]
struct ResultsView {
    copy: ActionButton,
    start_again: ActionButton,
    quit: ActionButton,
}

#[derive(Debug)]
pub struct App<'a> {
    cursor: Cursor,
    showing_export_popup: bool,
    showing_results_screen: bool,
    code: String,
    exit: bool,
    main_view: MainView<'a>,
    results_view: ResultsView,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            cursor: Cursor {
                section: Section::GameOptions,
                pos: 0,
            },
            showing_export_popup: false,
            showing_results_screen: false,
            code: String::new(),
            exit: false,
            main_view: MainView {
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
                generate: ActionButton::new("< Generate Code >"),
                export_popup: ExportPopup {
                    name: LabelledTextArea::new("Name: ", "My Custom mode"),
                    description: LabelledTextArea::new(
                        "Description: ",
                        "Fox only, no items, Final Destination",
                    ),
                    author: LabelledTextArea::new("Author: ", "John Melee"),
                    back: ActionButton::new("Back"),
                    confirm: ActionButton::new("Confirm"),
                },
            },
            results_view: ResultsView {
                copy: ActionButton::new("Copy to clipboard"),
                start_again: ActionButton::new("Start again"),
                quit: ActionButton::new("Quit"),
            },
        }
    }
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.update_selection();
        self.update_code();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_main_view(&mut self, frame: &mut Frame, area: Rect) {
        let main_layout = Layout::vertical(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(area);

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

        let action_buttons = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Length(21),
            Constraint::Fill(1),
        ])
        .split(main_layout[2]);

        frame.render_widget(game_options_block, main_layout[0]);
        frame.render_widget(&self.main_view.mode, game_options[0]);
        frame.render_widget(&self.main_view.stocks, game_options[1]);
        frame.render_widget(&self.main_view.time, game_options[2]);
        frame.render_widget(&self.main_view.item_frequency, game_options[3]);
        frame.render_widget(&self.main_view.stages, stages_and_items[0]);
        frame.render_widget(&self.main_view.items, stages_and_items[1]);
        frame.render_widget(&self.main_view.generate, action_buttons[1]);

        if self.showing_export_popup {
            let popup = area.centered(Constraint::Percentage(80), Constraint::Length(13));

            let popup_block = Block::bordered()
                .title("Code Options")
                .title_alignment(Alignment::Center)
                .border_style(Style::new().add_modifier(Modifier::ITALIC))
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 1,
                    bottom: 0,
                });

            let layout = Layout::vertical(vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(popup_block.inner(popup));

            let footer = Layout::horizontal(vec![
                Constraint::Fill(1),
                Constraint::Length(21),
                Constraint::Length(21),
                Constraint::Fill(1),
            ])
            .split(layout[4]);

            frame.render_widget(Clear, popup);
            frame.render_widget(popup_block, popup);
            frame.render_widget(&self.main_view.export_popup.name, layout[0]);
            frame.render_widget(&self.main_view.export_popup.description, layout[1]);
            frame.render_widget(&self.main_view.export_popup.author, layout[2]);
            frame.render_widget(&self.main_view.export_popup.back, footer[1]);
            frame.render_widget(&self.main_view.export_popup.confirm, footer[2]);
        }
    }

    fn render_results_view(&mut self, frame: &mut Frame, area: Rect) {
        let code = Paragraph::new(self.code.clone()).block(
            Block::bordered()
                .title("Your Code:")
                .padding(Padding::symmetric(1, 0)),
        );

        let layout = Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(3)]).split(area);

        let footer = Layout::horizontal(vec![
            Constraint::Length(25),
            Constraint::Fill(1),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Fill(1),
        ])
        .split(layout[1]);

        frame.render_widget(code, layout[0]);
        frame.render_widget(&self.results_view.copy, footer[0]);
        frame.render_widget(&self.results_view.start_again, footer[2]);
        frame.render_widget(&self.results_view.quit, footer[3]);
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

        let content_area = block.inner(frame.area());

        frame.render_widget(block, frame.area());

        if self.showing_results_screen {
            self.render_results_view(frame, content_area);
        } else {
            self.render_main_view(frame, content_area);
        }
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn current_section_rows(&self) -> usize {
        match self.cursor.section {
            Section::GameOptions => 4,
            Section::Stages => self.main_view.stages.entries.len(),
            Section::Items => self.main_view.items.entries.len(),
            Section::Footer => 1,
            Section::ExportOptions => 3,
            Section::ExportFooter => 2,
            Section::ResultsScreenFooter => 3,
        }
    }

    fn update_selection(&mut self) {
        self.cursor.pos = cmp::min(self.cursor.pos, self.current_section_rows() - 1);

        self.main_view.mode.selected = self.cursor.is_on(Section::GameOptions, 0);
        self.main_view.stocks.selected = self.cursor.is_on(Section::GameOptions, 1);
        self.main_view.time.selected = self.cursor.is_on(Section::GameOptions, 2);
        self.main_view.item_frequency.selected = self.cursor.is_on(Section::GameOptions, 3);

        for (index, entry) in self.main_view.stages.entries.iter_mut().enumerate() {
            entry.selected = self.cursor.is_on(Section::Stages, index);
        }

        for (index, entry) in self.main_view.items.entries.iter_mut().enumerate() {
            entry.selected = self.cursor.is_on(Section::Items, index);
        }

        self.main_view.generate.selected = self.cursor.is_on(Section::Footer, 0);

        let export_popup = &mut self.main_view.export_popup;
        export_popup.name.selected = self.cursor.is_on(Section::ExportOptions, 0);
        export_popup.description.selected = self.cursor.is_on(Section::ExportOptions, 1);
        export_popup.author.selected = self.cursor.is_on(Section::ExportOptions, 2);
        export_popup.back.selected = self.cursor.is_on(Section::ExportFooter, 0);
        export_popup.confirm.selected = self.cursor.is_on(Section::ExportFooter, 1);

        self.results_view.copy.selected = self.cursor.is_on(Section::ResultsScreenFooter, 0);
        self.results_view.start_again.selected = self.cursor.is_on(Section::ResultsScreenFooter, 1);
        self.results_view.quit.selected = self.cursor.is_on(Section::ResultsScreenFooter, 2);
    }

    fn update_code(&mut self) {
        let mode = match self.main_view.mode.current_state {
            1 => GameMode::Doubles,
            _ => GameMode::Direct,
        };

        let stocks = self.main_view.stocks.value;

        let time_limit: Option<u8> = match self.main_view.time.value {
            0 => None,
            limit => Some(limit),
        };

        let stages: Vec<code_generation::Bit> = self
            .main_view
            .stages
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| code_generation::Bit {
                pos: melee::default_stages()[index].bit,
                state: entry.checked,
            })
            .collect();

        let item_frequency = match self.main_view.item_frequency.current_state {
            1 => code_generation::ItemFrequency::VeryLow,
            2 => code_generation::ItemFrequency::Low,
            3 => code_generation::ItemFrequency::Medium,
            4 => code_generation::ItemFrequency::High,
            5 => code_generation::ItemFrequency::VeryHigh,
            _ => code_generation::ItemFrequency::None,
        };

        let items: Vec<code_generation::Bit> = self
            .main_view
            .items
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| code_generation::Bit {
                pos: melee::default_items()[index].bit,
                state: entry.checked,
            })
            .collect();

        self.code =
            code_generation::generate(mode, stocks, time_limit, stages, item_frequency, items);
    }

    fn increment_cursor(&mut self) {
        if self.cursor.pos < self.current_section_rows() - 1 {
            self.cursor.pos += 1;
            self.update_selection();
        }
    }

    fn decrement_cursor(&mut self) {
        if self.cursor.pos > 0 {
            self.cursor.pos -= 1;
            self.update_selection();
        }
    }

    fn update_cursor(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => match self.cursor.section.direction() {
                CursorDirection::Vertical => {
                    if self.cursor.pos == 0 {
                        if self.cursor.section == Section::Stages
                            || self.cursor.section == Section::Items
                        {
                            self.cursor.section = Section::GameOptions;
                        }
                    } else {
                        self.decrement_cursor();
                    }
                }
                CursorDirection::Horizontal => {
                    if self.cursor.section == Section::Footer {
                        self.cursor = Cursor {
                            section: Section::Items,
                            pos: self.current_section_rows() - 1,
                        };
                    } else if self.cursor.section == Section::ExportFooter {
                        self.cursor = Cursor {
                            section: Section::ExportOptions,
                            pos: self.current_section_rows() - 1,
                        };
                    }
                }
            },
            KeyCode::Down => match self.cursor.section.direction() {
                CursorDirection::Vertical => {
                    if self.cursor.pos >= (self.current_section_rows() - 1) {
                        if self.cursor.section == Section::Stages
                            || self.cursor.section == Section::Items
                        {
                            self.cursor.section = Section::Footer;
                        } else if self.cursor.section == Section::ExportOptions {
                            self.cursor.section = Section::ExportFooter;
                        }
                    } else {
                        self.increment_cursor();
                    }
                }
                CursorDirection::Horizontal => {
                    if self.cursor.section == Section::GameOptions {
                        self.cursor = Cursor {
                            section: Section::Stages,
                            pos: 0,
                        };
                    }
                }
            },
            KeyCode::Left => match self.cursor.section.direction() {
                CursorDirection::Vertical => {
                    if self.cursor.section == Section::Items {
                        self.cursor.section = Section::Stages;
                    }
                }
                CursorDirection::Horizontal => {
                    self.decrement_cursor();
                }
            },
            KeyCode::Right => match self.cursor.section.direction() {
                CursorDirection::Vertical => {
                    if self.cursor.section == Section::Stages {
                        self.cursor.section = Section::Items;
                    }
                }
                CursorDirection::Horizontal => {
                    self.increment_cursor();
                }
            },
            _ => {}
        }
    }

    fn handle_keys(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            let key = key_event.code;
            let mut handled: bool = false;
            match self.cursor.section {
                Section::GameOptions => {
                    match self.cursor.pos {
                        0 => handled = self.main_view.mode.handle_key_press(key),
                        1 => handled = self.main_view.stocks.handle_key_press(key),
                        2 => handled = self.main_view.time.handle_key_press(key),
                        3 => handled = self.main_view.item_frequency.handle_key_press(key),
                        _ => {}
                    };
                    if handled {
                        self.update_code()
                    };
                }
                Section::Stages => {
                    handled = self.main_view.stages.handle_key_press(key, self.cursor.pos);
                    if handled {
                        self.update_code()
                    };
                }
                Section::Items => {
                    handled = self.main_view.items.handle_key_press(key, self.cursor.pos);
                    if handled {
                        self.update_code()
                    };
                }
                Section::Footer => {
                    handled = self.main_view.generate.handle_key_press(key, || {
                        self.showing_export_popup = true;
                        self.cursor = Cursor {
                            section: Section::ExportOptions,
                            pos: 0,
                        };
                    });
                }
                Section::ExportOptions => {
                    if self.main_view.export_popup.name.selected {
                        handled = self.main_view.export_popup.name.handle_key_press(key_event);
                    }
                    if self.main_view.export_popup.description.selected {
                        handled = self
                            .main_view
                            .export_popup
                            .description
                            .handle_key_press(key_event);
                    }
                    if self.main_view.export_popup.author.selected {
                        handled = self
                            .main_view
                            .export_popup
                            .author
                            .handle_key_press(key_event);
                    }
                    if handled {
                        self.update_code();
                    }
                }
                Section::ExportFooter => {
                    if self.main_view.export_popup.back.selected {
                        handled = self.main_view.export_popup.back.handle_key_press(key, || {
                            self.showing_export_popup = false;
                            self.cursor = Cursor {
                                section: Section::Footer,
                                pos: 0,
                            };
                        });
                    }
                    if self.main_view.export_popup.confirm.selected {
                        handled = self
                            .main_view
                            .export_popup
                            .confirm
                            .handle_key_press(key, || {
                                self.showing_export_popup = false;
                                self.showing_results_screen = true;
                                self.cursor = Cursor {
                                    section: Section::ResultsScreenFooter,
                                    pos: 0,
                                };
                            });
                    }
                }
                Section::ResultsScreenFooter => {}
            }
            if !handled {
                if self.showing_export_popup && key == KeyCode::Esc {
                    self.showing_export_popup = false;
                    self.cursor = Cursor {
                        section: Section::Footer,
                        pos: 0,
                    };
                    handled = true;
                }
                if key == KeyCode::Char('q') {
                    self.quit();
                    handled = true;
                }
            }

            if !handled {
                self.update_cursor(key_event);
            }

            self.update_selection();
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
