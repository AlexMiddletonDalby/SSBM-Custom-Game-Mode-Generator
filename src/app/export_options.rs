use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui_textarea::TextArea;

#[derive(Debug)]
struct LabelledTextArea<'a> {
    label: String,
    text_area: TextArea<'a>,
}

impl<'a> LabelledTextArea<'a> {
    pub fn new(text: &str) -> Self {
        Self {
            label: text.to_string(),
            text_area: TextArea::default(),
        }
    }
}

impl<'a> Widget for &LabelledTextArea<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = Paragraph::new(self.label.clone());

        let layout =
            Layout::horizontal(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(area);

        label.render(layout[0], buf);
        self.text_area.render(layout[1], buf);
    }
}

#[derive(Debug)]
pub struct ExportOptionsPopup<'a> {
    name: LabelledTextArea<'a>,
    description: LabelledTextArea<'a>,
    author: LabelledTextArea<'a>,
}

impl<'a> ExportOptionsPopup<'a> {
    pub fn new() -> Self {
        Self {
            name: LabelledTextArea::new("Name: "),
            description: LabelledTextArea::new("Description: "),
            author: LabelledTextArea::new("Author: "),
        }
    }
}

impl<'a> Widget for &ExportOptionsPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::bordered()
            .title("Code Options")
            .title_alignment(Alignment::Center)
            .border_style(Style::new().add_modifier(Modifier::ITALIC))
            .padding(Padding::symmetric(1, 1));

        let layout = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(popup_block.inner(area));

        popup_block.render(area, buf);
        self.name.render(layout[0], buf);
        self.description.render(layout[1], buf);
        self.author.render(layout[2], buf);
    }
}
