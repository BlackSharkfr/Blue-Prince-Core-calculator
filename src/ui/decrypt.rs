use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Row, Table, TableState},
};

use crate::{
    calculator::{decrypt::decrypt, num_to_char},
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

    pub fn process_input(&mut self) {
        let Some(input) = self.prompt.event_enter() else {
            return;
        };

        let (cores, errors) = decrypt(&input);
        self.previous_queries.push(DecryptQuery {
            input,
            cores,
            errors,
        });

        self.history_table.select_last();
    }
}

struct DecryptQuery {
    input: String,
    cores: Vec<Option<u32>>,
    errors: Vec<String>,
}
impl DecryptQuery {
    fn output_text(&self) -> Line<'_> {
        let mut error_text = String::new();
        if !self.errors.is_empty() {
            error_text.push_str("Errors : ");
            for error in &self.errors {
                error_text.push_str(error.as_str());
            }
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
