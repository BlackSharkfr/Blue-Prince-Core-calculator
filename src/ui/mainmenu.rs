use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    prelude::*,
    symbols::border,
    widgets::{Block, ListDirection, ListState, Padding},
};

use crate::ui::{App, Mode};

pub struct MainMenu {
    pub state: ListState,
}
impl Default for MainMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select_first();
        MainMenu { state }
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

        Line::from(vec![
            "Blue Prince".light_blue(),
            " - Numeric Core calculator".into(),
        ])
        .centered()
        .bold()
        .render(title_bar, frame.buffer_mut());

        let list = ratatui::widgets::List::new(["Decrypt", "Encrypt", "Quit"])
            .highlight_symbol(">> ")
            .highlight_style(Style::default().bold())
            .direction(ListDirection::TopToBottom)
            .block(
                Block::bordered()
                    .title(Line::from(" Main menu ".bold()))
                    .border_set(border::THICK)
                    .padding(Padding::horizontal(1)),
            );
        StatefulWidget::render(list, menu_area, frame.buffer_mut(), &mut self.state);

        Line::from(vec![
            " Navigate ".into(),
            "<UP><DOWN>".blue().bold(),
            " | ".bold(),
            "Select ".into(),
            "<ENTER>".blue().bold(),
            " | ".bold(),
            "Quit ".into(),
            "<ESC> ".blue().bold(),
        ])
        .centered()
        .render(instructions_bar, frame.buffer_mut());
    }
}

pub fn handle_events(app: &mut App, event: Event) {
    match event {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Up => app.main_menu.state.select_previous(),
            KeyCode::Down => app.main_menu.state.select_next(),
            KeyCode::Enter => match app.main_menu.state.selected() {
                Some(0) => app.change_mode(Mode::Decrypt),
                Some(1) => app.change_mode(Mode::Encrypt),
                Some(2) => app.quit = true,
                _ => app.main_menu.state.select_first(),
            },
            KeyCode::Esc => app.quit = true,
            _ => (),
        },
        _ => (),
    }
}
