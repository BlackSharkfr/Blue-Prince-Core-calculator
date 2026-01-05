mod decryptmenu;
mod encryptmenu;
mod mainmenu;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEventKind},
    },
    layout::Rect,
    style::Stylize,
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

use crate::ui::{decryptmenu::Decrypt, encryptmenu::Encrypt, mainmenu::MainMenu};

#[derive(Default)]
pub struct App {
    mode: Mode,
    main_menu: MainMenu,
    decrypt: Decrypt,
    encrypt: Encrypt,
    quit: bool,
}

#[derive(Default)]
enum Mode {
    #[default]
    Menu,
    Decrypt,
    Encrypt,
}

enum ModeSelection {
    MainMenu,
    Decrypt,
    Encrypt,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match &mut self.mode {
            Mode::Menu => {
                self.main_menu.draw(frame);
            }
            Mode::Decrypt => self.decrypt.draw(frame),
            Mode::Encrypt => self.encrypt.draw(frame),
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match &mut self.mode {
            Mode::Menu => {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Up => self.main_menu.state.select_previous(),
                            KeyCode::Down => self.main_menu.state.select_next(),
                            KeyCode::Enter => match self.main_menu.state.selected() {
                                Some(0) => self.change_mode(ModeSelection::Decrypt),
                                Some(1) => self.change_mode(ModeSelection::Encrypt),
                                Some(2) => self.quit = true,
                                _ => self.main_menu.state.select_first(),
                            },
                            KeyCode::Esc => self.quit = true,
                            _ => (),
                        }
                    }
                    _ => (),
                }
                Ok(())
            }
            Mode::Decrypt => {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Esc => self.change_mode(ModeSelection::MainMenu),
                            KeyCode::Char(c) => self.decrypt.prompt.event_char(c),
                            KeyCode::Delete => self.decrypt.prompt.event_delete(),
                            KeyCode::Backspace => self.decrypt.prompt.event_backspace(),
                            KeyCode::Left => self.decrypt.prompt.event_left(),
                            KeyCode::Right => self.decrypt.prompt.event_right(),
                            KeyCode::Up => self.decrypt.history_up(),
                            KeyCode::Down => self.decrypt.history_down(),
                            KeyCode::Home => self.decrypt.prompt.event_home(),
                            KeyCode::End => self.decrypt.prompt.event_end(),
                            KeyCode::Enter => self.decrypt.input_submitted(),
                            _ => (),
                        }
                    }
                    _ => (),
                };
                Ok(())
            }
            Mode::Encrypt => {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Esc => self.change_mode(ModeSelection::MainMenu),
                            KeyCode::Char(c) => self.encrypt.prompt.event_char(c),
                            KeyCode::Delete => self.encrypt.prompt.event_delete(),
                            KeyCode::Backspace => self.encrypt.prompt.event_backspace(),
                            KeyCode::Left => self.encrypt.prompt.event_left(),
                            KeyCode::Right => self.encrypt.prompt.event_right(),
                            KeyCode::Home => self.encrypt.prompt.event_home(),
                            KeyCode::End => self.encrypt.prompt.event_end(),
                            KeyCode::Enter => self.encrypt.input_submitted(),
                            KeyCode::PageUp => self.encrypt.previous_page(),
                            KeyCode::PageDown => self.encrypt.next_page(),
                            _ => (),
                        }
                    }
                    _ => (),
                }
                Ok(())
            }
        }
    }

    fn change_mode(&mut self, selection: ModeSelection) {
        // FROM current code TO selection
        match (&self.mode, selection) {
            (Mode::Menu, ModeSelection::Decrypt) => {
                self.mode = Mode::Decrypt;
                self.decrypt.init();
            }
            (Mode::Menu, ModeSelection::Encrypt) => {
                self.mode = Mode::Encrypt;
                self.encrypt.init();
            }
            (Mode::Decrypt, ModeSelection::MainMenu) => {
                self.mode = Mode::Menu;
                self.main_menu.state.select(Some(0));
            }
            (Mode::Encrypt, ModeSelection::MainMenu) => {
                self.mode = Mode::Menu;
                self.main_menu.state.select(Some(1));
            }
            _ => unreachable!(),
        }
    }
}

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
        if !(c.is_ascii_alphanumeric() || c == ' ') {
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
