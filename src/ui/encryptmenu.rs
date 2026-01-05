use crate::{
    calculator::{CORE_LENGTH, encryptor::encrypt_letter},
    ui::Prompt,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Row, Table},
};

#[derive(Default)]
pub struct Encrypt {
    results: Option<Result<EncryptResults, String>>,
    page_start: usize,
    page_len: u16,
    pub prompt: Prompt,
}

struct EncryptResults {
    input: String,
    cyphers: Vec<[char; CORE_LENGTH]>,
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
                let col_width = CORE_LENGTH as u16 + 2;
                let table_rows = u16::max(1, results_area.height.saturating_sub(2));
                let table_cols = u16::max(1, results_area.width.saturating_sub(2) / col_width);
                let total_cols = 1 + (results.cyphers.len() / table_rows as usize);
                let total_pages = 1 + (total_cols / table_cols as usize);
                self.page_len = table_rows * table_cols;
                let current_page = self.page_start / self.page_len as usize + 1;
                self.page_start = self.page_start - (self.page_start % self.page_len as usize);
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
                                    self.page_start
                                        + row as usize
                                        + (col as usize * table_rows as usize),
                                )
                                .map(|chars| chars.iter().collect::<String>())
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

    pub fn input_submitted(&mut self) {
        let Some(input) = self.prompt.event_enter() else {
            return;
        };
        let input = input.trim();
        if input.len() != 1 {
            return;
        }
        let Some(c) = input.chars().next().filter(|c| c.is_ascii_alphabetic()) else {
            return;
        };
        let results = encrypt_letter(c).map(|cyphers| EncryptResults {
            input: input.to_string(),
            cyphers,
        });

        self.results = Some(results);
    }
    pub fn previous_page(&mut self) {
        self.page_start = self.page_start.saturating_sub(self.page_len as usize);
    }
    pub fn next_page(&mut self) {
        let Some(Ok(results)) = &mut self.results else {
            return;
        };
        let first_of_last_page =
            results.cyphers.len() - results.cyphers.len() % self.page_len as usize;
        self.page_start = usize::min(self.page_start + self.page_len as usize, first_of_last_page);
    }
}
