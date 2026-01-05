use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    prelude::*,
    symbols::border,
    widgets::{Block, ListDirection, ListState, Padding},
};

use crate::ui::{App, Mode};

pub struct MainMenu {
    pub list: ListState,
}
impl Default for MainMenu {
    fn default() -> Self {
        let list = ListState::default().with_selected(Some(0));
        MainMenu { list }
    }
}

impl MainMenu {
    pub fn draw(&mut self, frame: &mut Frame) {
        let [title_bar, menu_area, instructions_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let title = Line::from_iter([
            "Blue Prince".light_blue(),
            " - Numeric Core calculator".into(),
        ])
        .centered()
        .bold();
        title.render(title_bar, frame.buffer_mut());

        let list = ratatui::widgets::List::new(["Decrypt", "Encrypt", "Quit"])
            .highlight_symbol(">> ")
            .highlight_style(style::Modifier::BOLD)
            .direction(ListDirection::TopToBottom)
            .block(
                Block::bordered()
                    .title(Line::from(" Main menu ".bold()))
                    .border_set(border::THICK)
                    .padding(Padding::horizontal(1)),
            );
        StatefulWidget::render(list, menu_area, frame.buffer_mut(), &mut self.list);

        let instructions = Line::from_iter([
            " Navigate ".into(),
            "<UP><DOWN>".blue().bold(),
            " | ".bold(),
            "Select ".into(),
            "<ENTER>".blue().bold(),
            " | ".bold(),
            "Quit ".into(),
            "<ESC> ".blue().bold(),
        ])
        .centered();
        instructions.render(instructions_bar, frame.buffer_mut());
    }
}

pub fn handle_events(app: &mut App, event: Event) {
    match event {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Up => app.main_menu.list.select_previous(),
            KeyCode::Down => app.main_menu.list.select_next(),
            KeyCode::Enter => match app.main_menu.list.selected().map(Mode::from_menuselection) {
                None => app.main_menu.list.select_first(),
                Some(mode) => app.change_mode(mode),
            },
            KeyCode::Esc => app.change_mode(Mode::Quit),
            _ => (),
        },
        _ => (),
    }
}

impl Mode {
    fn from_menuselection(index: usize) -> Mode {
        match index {
            0 => Mode::Decrypt,
            1 => Mode::Encrypt,
            2 => Mode::Quit,
            other => unreachable!("Item {other} does not exist"),
        }
    }
}
