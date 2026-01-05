use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, ListDirection, ListState, Padding},
};

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
