use itertools::Itertools;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::{Block, Padding, Row, Table, TableState},
};

use crate::{
    calculator::{
        Letter,
        decryptor::{DecryptError, DecryptInput, decrypt_numbers, decrypt_word},
    },
    ui::{App, Mode, widgets::Prompt},
};

/// Sanitize user inputs : do not allow infinite history
const PREVIOUS_QUERIES_MAX_LEN: usize = 128;

/// Decrypt page state
#[derive(Default)]
pub struct Decrypt {
    history: Vec<DecryptResult>,
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
            Line::from_iter(["Blue Prince".bold().blue(), " - Core Decrypt".bold()]).centered();
        header.render(title_bar, frame.buffer_mut());

        let history_block = Block::bordered()
            .title(" Previous decryptions ")
            .padding(Padding::horizontal(1));

        let max_input_width = self.history.iter().map(|result| result.input.len()).max();

        let table = match max_input_width {
            None => Table::new([Row::default()], [Constraint::Fill(1)]),
            Some(width) => Table::new(
                self.history.iter().enumerate().map(|(index, result)| {
                    let style = if self.selected == Some(index) {
                        style::Modifier::REVERSED
                    } else {
                        Default::default()
                    };
                    Row::from_iter([
                        Text::from(result.input.clone()).italic(),
                        Text::from(result.output_text()),
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

        Line::from_iter([
            " Input : ".into(),
            "<4 numbers>".bold().blue(),
            " for core, or ".into(),
            "<Words>".blue().bold(),
            " for text".into(),
            " | ".bold(),
            "Compute ".into(),
            "<ENTER>".blue().bold(),
            " | ".bold(),
            "Navigate ".into(),
            "<UP><DOWN>".blue().bold(),
            " | ".bold(),
            "Main menu ".into(),
            "<ESC> ".blue().bold(),
        ])
        .centered()
        .render(instructions_bar, frame.buffer_mut());
    }

    fn history_up(&mut self) {
        let result = match self.selected {
            None => {
                let Some(result) = self.history.last() else {
                    return;
                };
                self.selected = Some(self.history.len() - 1);
                self.table_state.select_last();
                result
            }
            Some(index) => {
                let Some(new_index) = index.checked_sub(1) else {
                    return;
                };
                let Some(result) = self.history.get(new_index) else {
                    return;
                };
                self.selected = Some(new_index);
                self.table_state.select(Some(new_index));
                result
            }
        };
        self.prompt.set_input(&result.input)
    }

    fn history_down(&mut self) {
        let Some(index) = self.selected else {
            return;
        };
        let new_index = index + 1;

        let Some(result) = self.history.get(new_index) else {
            self.selected = None;
            self.prompt.clear();
            return;
        };
        self.selected = Some(new_index);
        self.table_state.select(Some(new_index));
        self.prompt.set_input(&result.input);
    }

    fn input_submitted(&mut self) {
        let Some(input) = self.prompt.submit() else {
            return;
        };

        let result = process_input(input);
        if self.history.len() > PREVIOUS_QUERIES_MAX_LEN {
            self.history.remove(0);
        }
        self.history.push(result);
        self.table_state.select_last();
        self.selected = None;
    }
}

fn process_input(input: String) -> DecryptResult {
    let mut result: DecryptResult = DecryptResult::new(input);

    match result.input.parse() {
        Err(e) => result.push_error(e.to_string()),
        Ok(DecryptInput::Numbers(numbers)) => {
            result.push_result(decrypt_numbers(numbers));
        }
        Ok(DecryptInput::Words(words)) => {
            for word in words {
                result.push_result(decrypt_word(&word));
            }
        }
    }

    result
}

/// Record of a user's text input and it's decryption
#[derive(Debug, PartialEq, Eq)]
struct DecryptResult {
    input: String,
    cores: Vec<Option<u32>>,
    errors: Vec<String>,
}
impl DecryptResult {
    fn new(input: String) -> Self {
        DecryptResult {
            input,
            cores: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// store a successful core
    fn push_core(&mut self, core: u32) {
        self.cores.push(Some(core));
    }

    /// store an error
    fn push_error(&mut self, error: String) {
        self.cores.push(None);
        self.errors.push(error);
    }

    /// automatically determine whether to store a core or an error
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
            .map(|core| match core.and_then(Letter::try_from_num) {
                Some(letter) => Span::from(letter.to_string()).green(),
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
            KeyCode::Char(c) => app.decrypt.prompt.input_char(c),
            KeyCode::Delete => app.decrypt.prompt.delete_right(),
            KeyCode::Backspace => app.decrypt.prompt.delete_left(),
            KeyCode::Left => app.decrypt.prompt.cursor_left(),
            KeyCode::Right => app.decrypt.prompt.cursor_right(),
            KeyCode::Up => app.decrypt.history_up(),
            KeyCode::Down => app.decrypt.history_down(),
            KeyCode::Home => app.decrypt.prompt.cursor_start(),
            KeyCode::End => app.decrypt.prompt.cursor_end(),
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
        let input = "PIGS SAND\r\nMAIL DATE\tHEAD".to_string();
        let result = process_input(input.clone());
        let expected = DecryptResult {
            input,
            cores: vec![Some(19), Some(20), Some(9), Some(12), Some(12)],
            errors: Vec::new(),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn known_numbers() {
        let input = "1000 200 11 2".to_string();
        let result = process_input(input.clone());
        let expected = DecryptResult {
            input,
            cores: vec![Some(53)],
            errors: Vec::new(),
        };
        assert_eq!(result, expected)
    }
}
