mod decryptmenu;
mod encryptmenu;
mod mainmenu;
mod widgets;

use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame, crossterm::event};

use crate::ui::{decryptmenu::Decrypt, encryptmenu::Encrypt, mainmenu::MainMenu};

pub fn run() -> Result<(), String> {
    ratatui::run(|terminal| App::default().run(terminal)).map_err(|io_error| io_error.to_string())
}

/// Application state
#[derive(Default)]
struct App {
    // States of pages
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
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
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
