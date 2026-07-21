use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::tui::{
    event::PageEventHandler,
    state::ApplicationState,
};
#[derive(Default)]
pub struct BatteryPageEventHandler;

impl PageEventHandler for BatteryPageEventHandler {
    fn handle_page_event(&self, key: KeyEvent, state: &mut ApplicationState) {
        // Handle battery page specific events here
        // up and down arrow keys should change the active input, left and right should change the value of the active input
        let active_input = &mut state.battery_page_state.active_input;
        match key.code {
            KeyCode::Up => {
                *active_input = active_input.prev();
            }

            KeyCode::Down => {
                *active_input = active_input.next();
            }

            KeyCode::Left => {
                state.battery_page_state.get_active_input_mut().decrement();
            }
            KeyCode::Right => {
                state.battery_page_state.get_active_input_mut().increment();
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                state.battery_page_state.get_active_input_mut().toggle();
            }
            _ => {}
        }
    }
}
