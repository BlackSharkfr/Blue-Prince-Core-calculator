mod decryptmenu;
mod encryptmenu;
mod mainmenu;

use crossterm::event::Event;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{cursor, event},
    layout::Rect,
    style::Stylize,
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

use crate::ui::{decryptmenu::Decrypt, encryptmenu::Encrypt, mainmenu::MainMenu};

/// Application state
#[derive(Default)]
pub struct App {
    /// States of pages
    main_menu: MainMenu,
    decrypt: Decrypt,
    encrypt: Encrypt,
    /// Current active page
    mode: Mode,
}

/// Current page being displayed
#[derive(Default, PartialEq, Eq)]
enum Mode {
    #[default]
    MainMenu,
    Decrypt,
    Encrypt,
    Quit,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.mode != Mode::Quit {
            terminal.draw(|frame| self.draw(frame))?;
            let event = event::read()?;
            self.handle_event(event);
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match &mut self.mode {
            Mode::MainMenu => {
                self.main_menu.draw(frame);
            }
            Mode::Decrypt => self.decrypt.draw(frame),
            Mode::Encrypt => self.encrypt.draw(frame),
            Mode::Quit => (),
        }
    }

    fn handle_event(&mut self, event: Event) {
        match &mut self.mode {
            Mode::MainMenu => mainmenu::handle_events(self, event),
            Mode::Decrypt => {
                decryptmenu::handle_events(self, event);
            }
            Mode::Encrypt => {
                encryptmenu::handle_events(self, event);
            }
            Mode::Quit => (),
        }
    }

    /// Change main application page from `self.mode` to `new_mode`
    fn change_mode(&mut self, new_mode: Mode) {
        match (&self.mode, &new_mode) {
            (Mode::MainMenu, Mode::Decrypt) => {
                self.mode = new_mode;
                self.decrypt.init();
            }
            (Mode::MainMenu, Mode::Encrypt) => {
                self.mode = new_mode;
                self.encrypt.init();
            }
            (Mode::MainMenu, Mode::Quit) => self.mode = Mode::Quit,
            (Mode::Decrypt, Mode::MainMenu) => {
                self.mode = new_mode;
                self.main_menu.list.select(Some(0));
            }
            (Mode::Encrypt, Mode::MainMenu) => {
                self.mode = new_mode;
                self.main_menu.list.select(Some(1));
            }
            _ => unreachable!(),
        }
    }
}

/// Sanitize user input : do not allow infinite input text
const PROMPT_MAX_LEN: usize = 192;

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

    pub fn set_focus(&mut self, bool: bool) {
        self.has_focus = bool;
        if bool {
            let mut stdout = std::io::stdout();
            ratatui::crossterm::execute!(stdout, cursor::SetCursorStyle::BlinkingBar).unwrap();
            ratatui::crossterm::execute!(stdout, cursor::EnableBlinking).unwrap();
        }
    }

    pub fn event_char(&mut self, c: char) {
        if !(c.is_ascii_alphanumeric() || c == ' ') || self.input.len() > PROMPT_MAX_LEN {
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

    pub fn event_backspace(&mut self) {
        if self.cursor_index == 0 {
            return;
        }
        self.cursor_index -= 1;
        self.input.remove(self.cursor_index);
    }

    pub fn event_delete(&mut self) {
        if self.cursor_index == self.input.len() {
            return;
        }
        self.input.remove(self.cursor_index);
    }

    pub fn event_left(&mut self) {
        self.cursor_index = self.cursor_index.saturating_sub(1);
    }
    pub fn event_right(&mut self) {
        self.cursor_index = usize::min(self.cursor_index + 1, self.input.len())
    }

    pub fn event_home(&mut self) {
        self.cursor_index = 0;
    }
    pub fn event_end(&mut self) {
        self.cursor_index = self.input.len();
    }

    pub fn event_enter(&mut self) -> Option<String> {
        if self.input.is_empty() {
            return None;
        }
        let output = self.input.trim().to_string();
        self.clear();
        Some(output)
    }
}
