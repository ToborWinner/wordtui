use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::Writing;

pub struct PopupWriting<'a> {
    title: &'a str,
    writing: &'a Writing,
}

impl<'a> PopupWriting<'a> {
    pub fn new(title: &'a str, writing: &'a Writing) -> Self {
        Self { title, writing }
    }
}

impl Widget for &PopupWriting<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3), // Index: 1
                Constraint::Fill(1),
            ])
            .split(area);

        let chunks_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Percentage(25),
                Constraint::Fill(1),
            ])
            .split(chunks[1]);

        let writing_block = Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .title_style(Style::default().fg(Color::Cyan))
            .border_style(Style::default().fg(Color::Cyan));

        let writing =
            Line::from("                                                         ".white())
                .centered();
        Paragraph::new(writing)
            .block(writing_block.clone())
            .render(chunks_horizontal[1], buf);

        let writing = Line::from(self.writing.text.to_string().white()).centered();
        Paragraph::new(writing)
            .block(writing_block)
            .render(chunks_horizontal[1], buf);
    }
}
