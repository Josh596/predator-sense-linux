use ratatui::{Frame, layout::Rect};

use crate::tui::{state::ApplicationState, utils::Action};

pub trait PageView {
    fn render(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState);
    fn actions(&self, state: &ApplicationState) -> Vec<Action>;
}
