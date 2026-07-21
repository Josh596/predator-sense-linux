use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Padding},
};

use crate::tui::{
    components::inputs::WidgetInput, state::ApplicationState, utils::Action,
    view::page::PageView,
};

#[derive(Default)]
pub struct PerformancePage;

impl PerformancePage {
    fn render_mode_input(frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        let block = Block::default()
            .title("PERFORMANCE MODE")
            .borders(Borders::ALL)
            .padding(Padding::uniform(2));

        let inner_area = block.inner(rect);

        frame.render_widget(block, rect);
        state.perf_page_state.mode_input.render(frame, inner_area);
    }

    fn render_active_profiler(frame: &mut Frame, rect: Rect, _state: &ApplicationState) {
        let block = Block::default()
            .title("ACTIVE PROFILE")
            .borders(Borders::ALL);

        frame.render_widget(block, rect);
    }
}

impl PageView for PerformancePage {
    fn render(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        let layout: [Rect; 2] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
            .spacing(4)
            .areas(rect);

        PerformancePage::render_mode_input(frame, layout[0], state);
        PerformancePage::render_active_profiler(frame, layout[1], state);
    }

    fn actions(&self, _state: &ApplicationState) -> Vec<Action> {
        vec![Action::new("↑↓", "Move")]
    }
}
