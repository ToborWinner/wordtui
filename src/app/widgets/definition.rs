use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget, Wrap},
};

use crate::words::Word;

pub struct Definition<'a> {
    word: &'a Word,
    current_scroll: u16,
}

impl<'a> Definition<'a> {
    pub fn new(word: &'a Word, current_scroll: u16) -> Self {
        Self {
            word,
            current_scroll,
        }
    }
}

impl Widget for &Definition<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Percentage(80),
                Constraint::Fill(1),
            ])
            .split(area);

        let word = Line::from(self.word.word.to_string().bold().yellow()).centered();

        let mut definition_text: Vec<Line> = self
            .word
            .extract
            .iter()
            .flat_map(|section| {
                let mut texts = vec![
                    Line::from(""),
                    Line::from(format!("[ {} ]", section.name).bold().yellow()).left_aligned(),
                ];

                let mut inside = false;
                for line in section.content.lines() {
                    if line.is_empty() {
                        continue;
                    }

                    texts.push(
                        Line::from(if line.starts_with("==== ") && line.ends_with(" ====") {
                            inside = true;
                            format!("  {}", &line[5..line.len() - 5]).bold().red()
                        } else if inside {
                            format!("    {}", line).into()
                        } else {
                            format!("  {}", line).into()
                        })
                        .left_aligned(),
                    );
                }

                texts
            })
            .collect();

        definition_text.insert(0, word);

        Paragraph::new(definition_text)
            .centered()
            .wrap(Wrap { trim: false })
            .scroll((self.current_scroll, 0))
            .render(chunks[1], buf);
    }
}
