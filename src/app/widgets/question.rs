use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{app::Writing, words::Word};

pub struct Question<'a> {
    word: &'a Word,
    writing: Option<&'a Writing>,
}

impl<'a> Question<'a> {
    pub fn new(word: &'a Word, writing: Option<&'a Writing>) -> Self {
        Self { word, writing }
    }
}

impl Widget for &Question<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1), // Index: 1
                Constraint::Length(2),
                Constraint::Length(3), // Index: 3
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
            .split(chunks[3]);

        let word = Line::from(self.word.word.to_string().bold().yellow()).centered();

        Paragraph::new(word).render(chunks[1], buf);

        let writing_block = Block::default()
            .borders(Borders::ALL)
            .title("Enter your answer")
            .title_style(Style::default().fg(if self.writing.is_some() {
                Color::Cyan
            } else {
                Color::Magenta
            }))
            .border_style(Style::default().fg(if self.writing.is_some() {
                Color::Cyan
            } else {
                Color::Magenta
            }));

        if let Some(writing) = self.writing {
            let writing = Line::from(writing.text.to_string().white()).centered();
            Paragraph::new(writing)
                .block(writing_block)
                .render(chunks_horizontal[1], buf);
        } else {
            writing_block.render(chunks_horizontal[1], buf);
        }
    }
}
