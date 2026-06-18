use ratatui::crossterm::event::{KeyCode, KeyEvent};
use strum::IntoEnumIterator;

use crate::tui::{
    event::PageEventHandler,
    state::{ApplicationState, battery::BatteryPageInput},
};
#[derive(Default)]
pub struct PerformancePageEventHandler;

impl PerformancePageEventHandler {
    fn modify_active_input(&self, state: &mut ApplicationState, increase: bool) {}
}

impl PageEventHandler for PerformancePageEventHandler {
    fn handle_page_event(&self, key: KeyEvent, state: &mut ApplicationState) {
        // Handle battery page specific events here
        // up and down arrow keys should change the active input, left and right should change the value of the active input
        let active_input = state.battery_page_state.active_input as usize;
        match key.code {
            KeyCode::Up => {
                let new_input = active_input.saturating_sub(1);
                state.battery_page_state.active_input =
                    BatteryPageInput::from_repr(new_input).expect("Invalid input index");
            }

            KeyCode::Down => {
                let new_input = active_input
                    .saturating_add(1)
                    .min(BatteryPageInput::iter().count() - 1);
                state.battery_page_state.active_input =
                    BatteryPageInput::from_repr(new_input).expect("Invalid input index");
            }

            KeyCode::Left => {
                state.battery_page_state.get_active_input_mut().decrement();
            }
            KeyCode::Right => {
                state.battery_page_state.get_active_input_mut().increment();
            }
            KeyCode::Char(' ') => {
                state.battery_page_state.get_active_input_mut().toggle();
            }
            _ => {}
        }
    }
}
