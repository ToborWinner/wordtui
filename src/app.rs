use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{cmp, io};
use widgets::{definition::Definition, popup_writing::PopupWriting, question::Question};

use crate::words::{Language, Word};

mod widgets;

#[derive(Debug)]
pub struct App {
    words: Vec<Word>,
    language_name: String,
    curr_index: usize,
    exit: bool,
    writing: Option<Writing>,
    current_screen: CurrentScreen,
    current_scroll: u16,
    last_correct: bool,
    known: usize,
    prev_streak: u32,
}

#[derive(Debug)]
pub struct Writing {
    text: String,
    special: bool,
}

impl Writing {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            special: false,
        }
    }

    pub fn toggle_special(&mut self) {
        self.special = !self.special;
    }

    pub fn push(&mut self, mut c: char) {
        if self.special {
            c = match c {
                'a' => 'ä',
                'o' => 'ö',
                'u' => 'ü',
                'A' => 'Ä',
                'O' => 'Ö',
                'U' => 'Ü',
                's' => 'ß',
                _ => c,
            };
            self.special = false;
        }

        self.text.push(c);
    }

    pub fn pop(&mut self) {
        self.text.pop();
    }
}

#[derive(Debug, PartialEq)]
enum CurrentScreen {
    Question,
    Definition,
}

impl App {
    pub fn new(language: Language) -> Self {
        let mut obj = Self {
            words: language.words,
            language_name: language.name,
            curr_index: 0,
            exit: false,
            writing: None,
            current_screen: CurrentScreen::Question,
            current_scroll: 0,
            last_correct: true,
            known: 0,
            prev_streak: 0,
        };

        obj.next_word();
        obj.calculate_known();

        obj
    }

    /// runs the application's main loop until the user quits
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<Language> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(Language {
            name: self.language_name,
            words: self.words,
        })
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn next_word(&mut self) {
        let first_wrong = self.words.iter().position(|word| word.streak < 5);
        if first_wrong.is_none() || rand::random::<u8>() % 5 == 0 {
            // Review
            self.curr_index = rand::random::<usize>() % (first_wrong.unwrap() + 1);
        } else {
            // Current batch
            let first_wrong = first_wrong.unwrap();
            let last = cmp::min(self.words.len() - 1, first_wrong + 10);
            self.curr_index = first_wrong + rand::random::<usize>() % (last - first_wrong + 1);
        }

        let word = self.words.get(self.curr_index).unwrap();
        if word.answer.is_some() {
            self.writing = Some(Writing::new());
            self.current_screen = CurrentScreen::Question;
        } else {
            self.writing = None;
            self.current_scroll = 0;
            self.current_screen = CurrentScreen::Definition;
        }
    }

    fn check_answer(&mut self) {
        let writing = self.writing.as_ref().unwrap();
        let answer = self
            .words
            .get(self.curr_index)
            .unwrap()
            .answer
            .as_ref()
            .unwrap();

        if writing.text == *answer {
            self.words[self.curr_index].streak += 1;
            self.words[self.curr_index].correct += 1;
            self.last_correct = true;
            self.calculate_known();
            self.next_word();
        } else {
            self.prev_streak = self.words[self.curr_index].streak;
            self.words[self.curr_index].streak = 0;
            self.words[self.curr_index].wrong += 1;
            self.last_correct = false;
            self.calculate_known();
            self.writing = None;
            self.current_screen = CurrentScreen::Definition;
            self.current_scroll = 0;
        }
    }

    fn set_answer(&mut self) {
        let writing = self.writing.as_ref().unwrap();
        self.words[self.curr_index].answer = Some(writing.text.clone());
        self.next_word();
    }

    fn calculate_known(&mut self) {
        self.known = self.words.iter().filter(|word| word.streak >= 5).count();
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match self.writing {
            Some(ref mut writing) => {
                match key_event.code {
                    KeyCode::Enter => match self.current_screen {
                        CurrentScreen::Question => self.check_answer(),
                        CurrentScreen::Definition => self.set_answer(),
                    },
                    KeyCode::Esc => self.writing = None,
                    KeyCode::Tab => writing.toggle_special(),
                    KeyCode::Backspace => writing.pop(),
                    KeyCode::Char(c) => writing.push(c),
                    _ => {}
                }
                return;
            }
            None => match self.current_screen {
                CurrentScreen::Question => match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('i') => self.writing = Some(Writing::new()),
                    KeyCode::Char('r') => {
                        self.writing = Some(Writing::new());
                        self.current_screen = CurrentScreen::Definition;
                    }
                    _ => {}
                },
                CurrentScreen::Definition => match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('k') => {
                        self.current_scroll = self.current_scroll.saturating_sub(1)
                    }
                    KeyCode::Char('j') => {
                        self.current_scroll = self.current_scroll.saturating_add(1)
                    }
                    KeyCode::Char('g') => self.current_scroll = 0,
                    KeyCode::Char('d') => {
                        self.current_scroll = self.current_scroll.saturating_add(10)
                    }
                    KeyCode::Char('u') => {
                        self.current_scroll = self.current_scroll.saturating_sub(10)
                    }
                    KeyCode::Char('f') => {
                        if self.last_correct {
                            return;
                        }
                        self.words[self.curr_index].streak = self.prev_streak;
                        self.words[self.curr_index].wrong -= 1;
                        self.words[self.curr_index].correct += 1;
                        self.last_correct = true;
                        self.calculate_known();
                    }
                    KeyCode::Char('n') => {
                        if self.words.get(self.curr_index).unwrap().answer.is_some() {
                            self.next_word()
                        } else {
                            self.writing = Some(Writing::new())
                        }
                    }
                    KeyCode::Char('r') => self.writing = Some(Writing::new()),
                    _ => {}
                },
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            format!("WORDTUI - {}", self.language_name),
            Style::default().fg(Color::Magenta),
        ))
        .centered()
        .block(title_block);

        title.render(chunks[0], buf);

        let footer_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let curr_word = self.words.get(self.curr_index).unwrap();

        Paragraph::new(
            Line::from(vec![
                "Streak: ".into(),
                curr_word.streak.to_string().bold().yellow(),
                " - Correct: ".into(),
                curr_word.correct.to_string().bold().green(),
                " - Wrong: ".into(),
                curr_word.wrong.to_string().bold().red(),
            ])
            .left_aligned(),
        )
        .block(footer_block.clone())
        .render(chunks[2], buf);

        Paragraph::new(
            if self.last_correct {
                "Correct".bold().green()
            } else {
                "Incorrect".bold().red()
            }
            .into_centered_line(),
        )
        .block(footer_block.clone())
        .render(chunks[2], buf);

        Paragraph::new(
            Line::from(vec![
                "Known: ".into(),
                self.known.to_string().bold().green(),
            ])
            .right_aligned(),
        )
        .block(footer_block)
        .render(chunks[2], buf);

        match self.current_screen {
            CurrentScreen::Question => Question::new(
                self.words.get(self.curr_index).unwrap(),
                self.writing.as_ref(),
            )
            .render(chunks[1], buf),
            CurrentScreen::Definition => Definition::new(
                self.words.get(self.curr_index).unwrap(),
                self.current_scroll,
            )
            .render(chunks[1], buf),
        }

        if let Some(writing) = self.writing.as_ref() {
            if self.current_screen == CurrentScreen::Definition {
                PopupWriting::new("Define the answer", writing).render(chunks[1], buf);
            }
        }
    }
}
