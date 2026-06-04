use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};
use strum::IntoEnumIterator;

use crate::{
    commands::power::PerfMode,
    tui::{
        app::{ApplicationState, performance::PerformancePage},
        components::pages::Page,
        utils::Action,
    },
};

impl PerformancePage {
    pub fn set_performance_mode(&mut self) {
        if let Some(i) = self.mode_list_state.selected() {
            let mode = PerfMode::iter().nth(i);

            // TODO: HANDLE EXCEPTIONS
            self.perfomance_mode = mode.unwrap();
        }
    }
}
impl Page for PerformancePage {
    fn actions(&self) -> impl IntoIterator<Item = Action> {
        vec![Action::new(String::from("↑↓"), String::from("Move"))]
    }

    fn render(&self, model: &mut ApplicationState, frame: &mut Frame, rect: Rect) {
        // All centered
        let [top, bottom] =
            Layout::vertical([Constraint::Length(10), Constraint::Fill(1)]).areas(rect);

        // Title

        let title = Line::from("Performance Profiles");

        frame.render_widget(title, top);

        let items: Vec<ListItem> = PerfMode::iter()
            .map(|mode| {
                let checkbox = if mode == model.performance_page.perfomance_mode {
                    Span::styled("[x]", Style::default().fg(Color::Green))
                } else {
                    Span::raw("[ ]") // Added a space here so the brackets align perfectly!
                };

                let list_item_ui =
                    Line::from(vec![checkbox, Span::raw(" "), Span::raw(mode.to_str())]);

                ListItem::new(list_item_ui)
            })
            .collect();

        let list = List::new(items);
        frame.render_stateful_widget(list, bottom, &mut model.performance_page.mode_list_state);
    }

    fn handle_event(&mut self, model: &mut ApplicationState, key: KeyEvent) {
        match key.code {
            KeyCode::Up => model.performance_page.mode_list_state.select_next(),
            KeyCode::Down => model.performance_page.mode_list_state.select_previous(),
            KeyCode::Enter => self.set_performance_mode(),
            _ => {}
        }
    }
}
