use itertools::Itertools;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::{Block, Padding, Row, Table, TableState},
};

use crate::{
    calculator::{
        CORE_LENGTH,
        decryptor::{DecryptError, decrypt_numbers, decrypt_word},
        num_to_char,
    },
    ui::{App, Mode, Prompt},
};

/// Sanitize user inputs : do not allow infinite history
const PREVIOUS_QUERIES_MAX_LEN: usize = 128;

/// Decrypt page state
#[derive(Default)]
pub struct Decrypt {
    history: Vec<DecryptQuery>,
    table_state: TableState,
    selected: Option<usize>,
    prompt: Prompt,
}

impl Decrypt {
    pub fn init(&mut self) {
        self.prompt.clear();
        self.prompt.set_focus(true);
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        let [title_bar, history_area, prompt_area, instructions_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let header =
            Line::from(vec!["Blue Prince".bold().blue(), " - Core Decrypt".bold()]).centered();
        header.render(title_bar, frame.buffer_mut());

        let history_block = Block::bordered()
            .title(" Previous decryptions ")
            .padding(Padding::horizontal(1));

        let max_query_width = self.history.iter().map(|query| query.input.len()).max();

        let table = match max_query_width {
            None => Table::new([Row::default()], [Constraint::Fill(1)]),
            Some(width) => Table::new(
                self.history.iter().enumerate().map(|(index, query)| {
                    let style = if self.selected == Some(index) {
                        style::Modifier::REVERSED
                    } else {
                        Default::default()
                    };
                    Row::new(vec![
                        Text::from(query.input.clone()).italic(),
                        Text::from(query.output_text()),
                    ])
                    .style(style)
                }),
                [Constraint::Length(width as u16), Constraint::Fill(1)],
            ),
        };

        let table = table
            .block(history_block)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(
            table,
            history_area,
            frame.buffer_mut(),
            &mut self.table_state,
        );

        self.prompt.draw(prompt_area, frame);

        Line::from(vec![
            " Input : ".into(),
            "<4 numbers>".bold().blue(),
            " for raw numeric core, or ".into(),
            "<Words>".blue().bold(),
            " for cyphertext".into(),
            " | ".bold(),
            "<ENTER>".blue().bold(),
            " compute".into(),
            " | ".bold(),
            "Navigate history ".into(),
            "<UP><DOWN>".blue().bold(),
            " | ".bold(),
            "Back to main menu ".into(),
            "<ESC> ".blue().bold(),
        ])
        .centered()
        .render(instructions_bar, frame.buffer_mut());
    }

    fn history_up(&mut self) {
        let query = match self.selected {
            None => {
                let Some(query) = self.history.last() else {
                    return;
                };
                self.selected = Some(self.history.len() - 1);
                self.table_state.select_last();
                query
            }
            Some(index) => {
                let Some(new_index) = index.checked_sub(1) else {
                    return;
                };
                let Some(query) = self.history.get(new_index) else {
                    return;
                };
                self.selected = Some(new_index);
                self.table_state.select(Some(new_index));
                query
            }
        };
        self.prompt.input = query.input.clone();
        self.prompt.event_end();
    }

    fn history_down(&mut self) {
        let Some(index) = self.selected else {
            return;
        };
        let new_index = index + 1;

        let Some(query) = self.history.get(new_index) else {
            self.selected = None;
            self.prompt.clear();
            return;
        };
        self.selected = Some(new_index);
        self.table_state.select(Some(new_index));
        self.prompt.input = query.input.clone();
        self.prompt.event_end();
    }

    fn input_submitted(&mut self) {
        let Some(input) = self.prompt.event_enter() else {
            return;
        };
        let input = input.trim();
        if input.is_empty() {
            return;
        }

        self.selected = None;

        let decrypt_query = process_input(input);
        if self.history.len() > PREVIOUS_QUERIES_MAX_LEN {
            self.history.remove(0);
        }
        self.history.push(decrypt_query);
        self.table_state.select_last();
    }
}

fn process_input(input: &str) -> DecryptQuery {
    let mut is_digits = false;
    let mut is_alphabetic = false;
    let words = input
        .split_whitespace()
        .inspect(|str| {
            for c in str.chars() {
                if c.is_ascii_digit() {
                    is_digits = true;
                } else if c.is_ascii_alphabetic() {
                    is_alphabetic = true;
                }
            }
        })
        .collect::<Vec<_>>();

    let mut decrypt_query = DecryptQuery::new(input.trim());
    match (is_digits, is_alphabetic) {
        (true, false) if words.len() == CORE_LENGTH => {
            // 4 numbers
            let mut numbers = [0; CORE_LENGTH];
            for (idx, word) in words.into_iter().enumerate() {
                let Ok(num) = word.parse::<u32>() else {
                    decrypt_query.push_error(format!("Failed to parse number '{word}'"));
                    continue;
                };
                numbers[idx] = num;
            }
            if decrypt_query.errors.is_empty() {
                decrypt_query.push_result(decrypt_numbers(numbers));
            }
        }
        (false, true) => {
            // 1 or many words
            for str in words {
                decrypt_query.push_result(decrypt_word(str));
            }
        }
        _ => decrypt_query
            .push_error("Invalid characters : expected 4 numbers or 4-letter words".into()),
    }

    decrypt_query
}

#[derive(Debug, PartialEq, Eq)]
struct DecryptQuery {
    input: String,
    cores: Vec<Option<u32>>,
    errors: Vec<String>,
}
impl DecryptQuery {
    fn new(input: &str) -> Self {
        DecryptQuery {
            input: input.trim().to_string(),
            cores: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn push_core(&mut self, core: u32) {
        self.cores.push(Some(core));
    }
    fn push_error(&mut self, error: String) {
        self.cores.push(None);
        self.errors.push(error);
    }
    fn push_result(&mut self, result: Result<u32, DecryptError>) {
        match result {
            Ok(core) => self.push_core(core),
            Err(error) => self.push_error(error.to_string()),
        }
    }

    fn output_text(&self) -> Line<'_> {
        let errors_header = match self.errors.len() {
            0 => Span::default(),
            1 => Span::from("Error : "),
            _ => Span::from("Errors : "),
        }
        .red();
        let errors = self.errors.iter().map(|e| Span::from(e).red());
        let errors = Itertools::intersperse(errors, Span::from(", ").red());

        if self.errors.len() == self.cores.len() {
            return Line::from_iter(std::iter::once(errors_header).chain(errors));
        }

        let values_header = match self.cores.len() {
            0 => Span::default(),
            1 => Span::from("Value : "),
            _ => Span::from("Values : "),
        }
        .green();
        let values = self.cores.iter().map(|core| match core {
            Some(number) => Span::from(number.to_string()).green(),
            None => Span::from("?").red(),
        });
        let values = Itertools::intersperse(values, Span::from(", ").green());

        let text_header = match self.cores.len() {
            0 => Span::default(),
            _ => Span::from(". Text : "),
        }
        .green();
        let text = self
            .cores
            .iter()
            .map(|core| match core.and_then(num_to_char) {
                Some(c) => Span::from(c.to_string()).green(),
                None => {
                    let span = Span::from("?");
                    match core.is_some() {
                        true => span.green(),
                        false => span.red(),
                    }
                }
            });

        let text_footer = match self.errors.len() {
            0 => Span::default(),
            _ => Span::from(". ").green(),
        };

        Line::from_iter(
            std::iter::once(values_header)
                .chain(values)
                .chain([text_header])
                .chain(text)
                .chain([text_footer])
                .chain([errors_header])
                .chain(errors),
        )
    }
}

pub fn handle_events(app: &mut App, event: Event) {
    match event {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Esc => app.change_mode(Mode::MainMenu),
            KeyCode::Char(c) => app.decrypt.prompt.event_char(c),
            KeyCode::Delete => app.decrypt.prompt.event_delete(),
            KeyCode::Backspace => app.decrypt.prompt.event_backspace(),
            KeyCode::Left => app.decrypt.prompt.event_left(),
            KeyCode::Right => app.decrypt.prompt.event_right(),
            KeyCode::Up => app.decrypt.history_up(),
            KeyCode::Down => app.decrypt.history_down(),
            KeyCode::Home => app.decrypt.prompt.event_home(),
            KeyCode::End => app.decrypt.prompt.event_end(),
            KeyCode::Enter => app.decrypt.input_submitted(),
            _ => (),
        },
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_words() {
        let input = " PIGS SAND\r\nMAIL DATE\tHEAD ";
        let decrypt_query = process_input(input);
        let expected = DecryptQuery {
            input: input.trim().to_string(),
            cores: vec![Some(19), Some(20), Some(9), Some(12), Some(12)],
            errors: Vec::new(),
        };
        assert_eq!(decrypt_query, expected);
    }

    #[test]
    fn known_numbers() {
        let input = "1000 200 11 2";
        let decript_query = process_input(input);
        let expected = DecryptQuery {
            input: input.to_string(),
            cores: vec![Some(53)],
            errors: Vec::new(),
        };
        assert_eq!(decript_query, expected)
    }
}
