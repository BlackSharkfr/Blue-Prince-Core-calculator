use crossterm::cursor;
use ratatui::{
    Frame,
    layout::Rect,
    prelude::*,
    style::Stylize,
    text::Span,
    widgets::{Block, Paragraph},
};

/**
    Simple widget for an input text prompt

    I didn't find on in ratatui's default widgets so I made this very basic one
*/
#[derive(Default)]
pub struct Prompt {
    input: String,
    cursor_index: usize,
    has_focus: bool,
}

impl Prompt {
    /// Sanitize user input : do not allow infinite input text
    const MAX_INPUT_LEN: usize = 192;

    pub fn draw(&self, area: Rect, frame: &mut Frame) {
        let block = Block::bordered().title(" Input prompt ");
        let pos = area.as_position();

        let mut text = Span::from(format!(" >> {}", self.input));
        if self.has_focus {
            text = text.bold();
            frame.set_cursor_position((pos.x + 5 + self.cursor_index as u16, pos.y + 1));
        }
        Paragraph::new(text)
            .block(block)
            .render(area, frame.buffer_mut());
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor_index = 0;
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
        self.cursor_index = input.len();
    }

    pub fn set_focus(&mut self, bool: bool) {
        self.has_focus = bool;
        if bool {
            let mut stdout = std::io::stdout();
            ratatui::crossterm::execute!(stdout, cursor::SetCursorStyle::BlinkingBar).unwrap();
            ratatui::crossterm::execute!(stdout, cursor::EnableBlinking).unwrap();
        }
    }

    pub fn input_char(&mut self, c: char) {
        if !(c.is_ascii_alphanumeric() || c == ' ') || self.input.len() > Self::MAX_INPUT_LEN {
            return;
        }
        let c = c.to_ascii_uppercase();

        if self.cursor_index == self.input.len() {
            self.input.push(c);
        } else {
            let (before, after) = self.input.split_at(self.cursor_index);
            self.input = format!("{before}{c}{after}");
        }
        self.cursor_index += 1
    }

    pub fn delete_left(&mut self) {
        if self.cursor_index == 0 {
            return;
        }
        self.cursor_index -= 1;
        self.input.remove(self.cursor_index);
    }

    pub fn delete_right(&mut self) {
        if self.cursor_index == self.input.len() {
            return;
        }
        self.input.remove(self.cursor_index);
    }

    pub fn cursor_left(&mut self) {
        self.cursor_index = self.cursor_index.saturating_sub(1);
    }

    pub fn cursor_right(&mut self) {
        self.cursor_index = usize::min(self.cursor_index + 1, self.input.len())
    }

    pub fn cursor_start(&mut self) {
        self.cursor_index = 0;
    }

    pub fn cursor_end(&mut self) {
        self.cursor_index = self.input.len();
    }

    /// Returns a trimmed input string. `None` if empty
    pub fn submit(&mut self) -> Option<String> {
        if self.input.is_empty() {
            return None;
        }
        let output = self.input.trim().to_string();
        self.clear();
        Some(output)
    }
}
