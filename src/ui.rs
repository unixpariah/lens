mod components;

use self::components::{current_command, mode, vi_bar};
use crate::{app::App, app::Window};
use components::{preview, results, search, text_area};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::ListState,
    Frame,
};

struct Colors {
    search: Color,
    command: Color,
}

impl Colors {
    fn new(chosen_window: &Window) -> Self {
        Self {
            search: if chosen_window == &Window::Search {
                Color::LightBlue
            } else {
                Color::Blue
            },
            command: if chosen_window == &Window::Command {
                Color::LightBlue
            } else {
                Color::Blue
            },
        }
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let colors = Colors::new(&app.window);
    let mut result_state = ListState::default();
    result_state.select(Some(app.search.scroll));

    let mut options_state = ListState::default();
    options_state.select(Some(app.search.scroll));

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main area
            Constraint::Length(2), // Footer
        ])
        .split(frame.size());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Search, and results column
            Constraint::Percentage(35), // Preview column
        ])
        .split(areas[0]);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search row
            Constraint::Min(0),    // Results row
        ])
        .split(columns[0]);

    frame.render_stateful_widget(results(app), rows[1], &mut result_state);
    frame.render_widget(text_area(app).widget(), rows[0]);
    frame.render_widget(search(colors.search), rows[0]);
    frame.render_widget(preview(app), columns[1]);
    frame.render_widget(vi_bar(app, colors.command).widget(), areas[1]);
    frame.render_widget(mode(app), areas[1]);
    frame.render_widget(current_command(app), areas[1]);
}
