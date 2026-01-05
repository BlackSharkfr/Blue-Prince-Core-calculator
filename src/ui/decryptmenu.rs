use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Row, Table, TableState},
};

use crate::{
    calculator::{
        decryptor::{DecryptError, LEN_DIGITS, decrypt_numbers, decrypt_word},
        num_to_char,
    },
    ui::Prompt,
};

#[derive(Default)]
pub struct Decrypt {
    previous_queries: Vec<DecryptQuery>,
    history_table: TableState,
    pub prompt: Prompt,
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

        Line::from(vec!["Blue Prince".bold().blue(), " - Decrypt".bold()])
            .centered()
            .render(title_bar, frame.buffer_mut());

        let history_block = Block::bordered()
            .title(" Previous decryptions ")
            .padding(Padding::horizontal(1));

        let max_query_width = self
            .previous_queries
            .iter()
            .map(|query| query.input.len())
            .max();

        let table = match max_query_width {
            None => Table::new([Row::default()], [Constraint::Fill(1)]),
            Some(width) => Table::new(
                self.previous_queries.iter().map(|query| {
                    Row::new(vec![
                        Text::from(query.input.clone()).italic(),
                        Text::from(query.output_text()),
                    ])
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
            &mut self.history_table,
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

    pub fn history_up(&mut self) {
        self.history_table.scroll_up_by(1);
    }

    pub fn history_down(&mut self) {
        self.history_table.scroll_down_by(1);
    }

    pub fn input_submitted(&mut self) {
        let Some(input) = self.prompt.event_enter() else {
            return;
        };
        let input = input.trim();
        if input.is_empty() {
            return;
        }

        let decrypt_query = process_input(input);
        self.previous_queries.push(decrypt_query);
        self.history_table.select_last();
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
        (true, false) if words.len() == LEN_DIGITS => {
            // 4 numbers
            let mut numbers = [0; LEN_DIGITS];
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
        let mut error_text = String::new();
        if !self.errors.is_empty() {
            error_text = format!("Errors : {}", self.errors.join(" "));
        }
        let mut text = String::new();
        let mut numbers = String::new();
        for result in &self.cores {
            match result {
                Some(core @ 1..=26) => {
                    text.push(num_to_char(*core));
                    numbers.push_str(&format!("{core}, "));
                }
                Some(core) => {
                    text.push('?');
                    numbers.push_str(&format!("{core}, "));
                }
                None => {
                    text.push('?');
                    numbers.push_str("?, ");
                }
            }
        }
        let mut output = Line::default();
        if self.cores.iter().any(Option::is_some) {
            output.push_span(Span::from(format!("Value : {numbers} Text : {text} ")).green());
        }
        output.push_span(Span::from(error_text).red());
        output
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
