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
    tui::{app::ApplicationState, components::pages::PageUI, utils::Action},
};

pub struct PerfomancePageUI;

impl PageUI for PerfomancePageUI {
    fn actions(&self) -> impl IntoIterator<Item = Action> {
        vec![Action::new("↑↓", "Move")]
    }

    fn render(&self, model: &mut ApplicationState, frame: &mut Frame, rect: Rect) {
        // All centered
        let [top, bottom] =
            Layout::vertical([Constraint::Length(10), Constraint::Fill(1)]).areas(rect);

        // Title

        let title = Line::from("Performance Profiles");

        let items: Vec<ListItem> = PerfMode::iter()
            .map(|mode| {
                let checkbox = if mode == model.performance_state.perfomance_mode {
                    Span::styled("[x]", Style::default().fg(Color::Green))
                } else {
                    Span::raw("[ ]") // Added a space here so the brackets align perfectly!
                };

                let list_item_ui =
                    Line::from(vec![checkbox, Span::raw(" "), Span::raw(mode.to_str())]);

                ListItem::new(list_item_ui)
            })
            .collect();

        let list = List::new(items)
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Yellow));

        frame.render_widget(title, top);
        frame.render_stateful_widget(list, bottom, &mut model.performance_state.mode_list_state);
    }

    fn handle_event(&mut self, model: &mut ApplicationState, key: &KeyEvent) {
        match key.code {
            KeyCode::Up => model.performance_state.mode_list_state.select_previous(),
            KeyCode::Down => model.performance_state.mode_list_state.select_next(),
            KeyCode::Enter => model.performance_state.set_performance_mode(),
            _ => {}
        }
    }
}
