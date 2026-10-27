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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::state::battery::BatteryPageInput;

    fn press(state: &mut ApplicationState, code: KeyCode) {
        BatteryPageEventHandler.handle_page_event(KeyEvent::from(code), state);
    }

    #[test]
    fn down_moves_to_next_input() {
        let mut state = ApplicationState::default();
        press(&mut state, KeyCode::Down);
        assert_eq!(
            state.battery_page_state.active_input,
            BatteryPageInput::UpperChargingLimit
        );
    }

    #[test]
    fn up_moves_to_previous_input() {
        let mut state = ApplicationState::default();
        state.battery_page_state.active_input = BatteryPageInput::LowerChargingLimit;
        press(&mut state, KeyCode::Up);
        assert_eq!(
            state.battery_page_state.active_input,
            BatteryPageInput::UpperChargingLimit
        );
    }

    #[test]
    fn left_decrements_active_input() {
        let mut state = ApplicationState::default();
        state.battery_page_state.active_input = BatteryPageInput::UpperChargingLimit;
        press(&mut state, KeyCode::Left);
        assert_eq!(state.battery_page_state.upper_charging_limit.value, 99);
    }

    #[test]
    fn right_increments_active_input() {
        let mut state = ApplicationState::default();
        state.battery_page_state.active_input = BatteryPageInput::LowerChargingLimit;
        press(&mut state, KeyCode::Right);
        assert_eq!(state.battery_page_state.lower_charging_limit.value, 1);
    }

    #[test]
    fn space_and_enter_toggle_active_input() {
        let mut state = ApplicationState::default();
        press(&mut state, KeyCode::Char(' '));
        assert!(state.battery_page_state.enable_charging_limit.value);
        press(&mut state, KeyCode::Enter);
        assert!(!state.battery_page_state.enable_charging_limit.value);
    }
}
