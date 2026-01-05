use crate::{
    calculator::{
        Cypher,
        encryptor::{encrypt_letter, encrypt_number},
    },
    ui::Prompt,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Row, Table},
};

#[derive(Default)]
pub struct Encrypt {
    results: Option<Result<EncryptResults, String>>,
    pub prompt: Prompt,
}

struct EncryptResults {
    input: String,
    cyphers: Vec<Cypher>,
    current_index: usize,
    items_per_page: u16,
    max_width: u16,
}
impl EncryptResults {
    fn compute_max_width(cyphers: &[Cypher]) -> u16 {
        let mut max_width = 1;
        for cypher in cyphers {
            max_width = u16::max(max_width, cypher.to_string().len() as u16)
        }
        max_width
    }
}

impl Encrypt {
    pub fn init(&mut self) {
        self.prompt.clear();
        self.prompt.set_focus(true);
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let [title_bar, results_area, prompt_area, instructions_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        Line::from(vec!["Blue Prince".bold().blue(), " - Encrypt".bold()])
            .centered()
            .render(title_bar, frame.buffer_mut());

        let block_title = match &self.results {
            Some(Ok(results)) => format!(" Possible encryptions for : '{}' ", results.input),
            _ => " Possible encryptions ".to_string(),
        };
        let mut results_block = Block::bordered()
            .title(block_title)
            .padding(Padding::horizontal(1));

        let table = match &mut self.results {
            None => Table::default(),
            Some(Err(e)) => Table::new([Row::new([e.as_str()])], [Constraint::Fill(1)]),
            Some(Ok(results)) => {
                let col_width = results.max_width + 2;
                let table_rows = u16::max(1, results_area.height.saturating_sub(2));
                let table_cols = u16::max(1, results_area.width.saturating_sub(2) / col_width);
                let total_cols = 1 + (results.cyphers.len() / table_rows as usize);
                let total_pages = 1 + (total_cols / table_cols as usize);
                results.items_per_page = table_rows * table_cols;
                let current_page = results.current_index / results.items_per_page as usize + 1;
                results.current_index = results.current_index
                    - (results.current_index % results.items_per_page as usize);
                results_block = results_block.title_bottom(
                    Line::from(format!(" Page : {current_page} of {total_pages} ",))
                        .right_aligned(),
                );
                Table::new(
                    (0..table_rows).map(|row| {
                        Row::new((0..table_cols).filter_map(|col| {
                            results
                                .cyphers
                                .get(
                                    results.current_index
                                        + row as usize
                                        + (col as usize * table_rows as usize),
                                )
                                .map(|cypher| cypher.to_string())
                        }))
                    }),
                    std::iter::repeat_n(Constraint::Length(col_width), table_cols as usize),
                )
            }
        };
        let table = table.block(results_block);
        Widget::render(table, results_area, frame.buffer_mut());

        self.prompt.draw(prompt_area, frame);

        Line::from(vec![
            " Input : ".into(),
            "<Number>".bold().blue(),
            " for raw numeric core, or ".into(),
            "<Letter>".blue().bold(),
            " for cyphertext".into(),
            " | ".bold(),
            "<ENTER>".blue().bold(),
            " compute".into(),
            " | ".bold(),
            "Navigate encryptions ".into(),
            "<PAGE UP><PAGE DOWN>".blue().bold(),
            " | ".bold(),
            "Back to main menu ".into(),
            "<ESC> ".blue().bold(),
        ])
        .centered()
        .render(instructions_bar, frame.buffer_mut());
    }

    pub fn process_input(&mut self) {
        self.results = self.prompt.event_enter().map(|input| {
            let result = if let Ok(number) = input.parse::<u32>() {
                encrypt_number(number)
            } else {
                encrypt_letter(&input)
            };

            result.map(|cyphers| {
                let max_width = EncryptResults::compute_max_width(&cyphers);
                EncryptResults {
                    input,
                    cyphers,
                    current_index: 0,
                    items_per_page: 1,
                    max_width,
                }
            })
        })
    }
    pub fn previous_page(&mut self) {
        let Some(Ok(results)) = &mut self.results else {
            return;
        };
        results.current_index = results
            .current_index
            .saturating_sub(results.items_per_page as usize);
    }
    pub fn next_page(&mut self) {
        let Some(Ok(results)) = &mut self.results else {
            return;
        };
        let first_of_last_page =
            results.cyphers.len() - results.cyphers.len() % results.items_per_page as usize;
        results.current_index = usize::min(
            results.current_index + results.items_per_page as usize,
            first_of_last_page,
        );
    }
}
