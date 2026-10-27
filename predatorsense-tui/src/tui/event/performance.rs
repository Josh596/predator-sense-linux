use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::tui::{
    event::PageEventHandler,
    state::{ApplicationState, fields::Field},
};
#[derive(Default)]
pub struct PerformancePageEventHandler;

impl PerformancePageEventHandler {
    fn modify_active_input(&self, _state: &mut ApplicationState, _increase: bool) {}
}

impl PageEventHandler for PerformancePageEventHandler {
    fn handle_page_event(&self, key: KeyEvent, state: &mut ApplicationState) {
        // Handle battery page specific events here
        // up and down arrow keys should change the active input, left and right should change the value of the active input
        let _active_input = state.battery_page_state.active_input as usize;
        match key.code {
            KeyCode::Up => {
                state.perf_page_state.mode_input.decrement();
            }

            KeyCode::Down => {
                state.perf_page_state.mode_input.increment();
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(state: &mut ApplicationState, code: KeyCode) {
        PerformancePageEventHandler.handle_page_event(KeyEvent::from(code), state);
    }

    #[test]
    fn down_selects_next_mode() {
        let mut state = ApplicationState::default(); // Normal, index 2
        press(&mut state, KeyCode::Down);
        assert_eq!(state.perf_page_state.mode_input.state.selected(), Some(3));
    }

    #[test]
    fn up_selects_previous_mode() {
        let mut state = ApplicationState::default();
        press(&mut state, KeyCode::Up);
        assert_eq!(state.perf_page_state.mode_input.state.selected(), Some(1));
    }
}
