use crate::tui::{app::ApplicationState, utils::Action};
use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};
// Create a trait called Pages that implement a actions() and render() method. The action method should return an Action Struct, while the
pub trait PageUI {
    fn actions(&self) -> Vec<Action>;
    fn render(&self, model: &mut ApplicationState, frame: &mut Frame, rect: Rect);
    fn handle_event(&mut self, model: &mut ApplicationState, key: &KeyEvent);
}

pub mod battery;
pub mod performance;
