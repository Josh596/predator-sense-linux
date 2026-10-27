use predatorsense::config::Config;
use ratatui::crossterm::event::{
    Event::{self},
    KeyCode, KeyEvent,
};

use crate::{
    services::{self, Applied},
    tui::state::{
        ApplicationState,
        Page::{self},
        RunningState,
    },
};
pub mod battery;
pub mod lighting;
pub mod performance;

#[derive(Default)]
pub struct EventHandler {
    battery_page_event_handler: battery::BatteryPageEventHandler,
    performance_page_event_handler: performance::PerformancePageEventHandler,
    lighting_page_event_handler: lighting::LightingPageEventHandler,
}

impl EventHandler {
    pub fn handle_event(&self, event: Event, state: &mut ApplicationState, config: &Config) {
        let old_simple_state = Applied::desired(state);

        match event {
            Event::Key(key) => {
                if self.get_active_page_event_handler(state).modal_open(state) {
                    self.get_active_page_event_handler(state)
                        .handle_modal_event(key, state);
                } else {
                    self.handle_key(key, state);

                    self.get_active_page_event_handler(state)
                        .handle_page_event(key, state);
                }
            }
            // After handling key events, delegate to the active page's event handler
            _ => {}
        }
        let new_simple_state = Applied::desired(state);

        if let Err(e) = services::execute(Some(old_simple_state), new_simple_state, config) {
            log::error!("could not apply state to the hardware: {e}");
        }

        return;
    }

    fn get_active_page_event_handler(&self, state: &ApplicationState) -> &dyn PageEventHandler {
        match state.active_page {
            Page::Battery => &self.battery_page_event_handler,
            Page::Performance => &self.performance_page_event_handler,
            Page::Lighting => &self.lighting_page_event_handler,
            _ => &self.battery_page_event_handler, // Default to BatteryPageEventHandler for now
        }
    }

    fn handle_key(&self, key: KeyEvent, state: &mut ApplicationState) {
        match key.code {
            KeyCode::Char('q') => {
                // Handle quit event
                state.running_state = RunningState::Done;
            }
            KeyCode::Char(c @ '1'..='9') if c.is_ascii_digit() => {
                let digit = c.to_digit(10).expect("Unexpected non-digit character");
                self.set_active_page((digit as usize).saturating_sub(1), state);
            }
            _ => {}
        }
    }

    fn set_active_page(&self, index: usize, state: &mut ApplicationState) {
        if let Some(page) = Page::from_repr(index) {
            state.active_page = page;
        }
    }
}

trait PageEventHandler {
    fn handle_page_event(&self, key: KeyEvent, state: &mut ApplicationState);
    fn modal_open(&self, _state: &ApplicationState) -> bool {
        false
    }
    fn handle_modal_event(&self, _key: KeyEvent, _state: &mut ApplicationState) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::state::color_picker::ColorPickerState;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::from(code)
    }

    // handle_key

    #[test]
    fn handle_key_q_quits() {
        let mut state = ApplicationState::default();
        EventHandler::default().handle_key(key(KeyCode::Char('q')), &mut state);
        assert_eq!(state.running_state, RunningState::Done);
    }

    #[test]
    fn handle_key_digits_switch_page() {
        let handler = EventHandler::default();
        let mut state = ApplicationState::default();
        for (digit, page) in [
            ('1', Page::Performance),
            ('2', Page::Battery),
            ('3', Page::Lighting),
        ] {
            handler.handle_key(key(KeyCode::Char(digit)), &mut state);
            assert_eq!(state.active_page, page, "pressed {digit}");
        }
    }

    #[test]
    fn handle_key_zero_is_ignored() {
        let mut state = ApplicationState::default();
        state.active_page = Page::Lighting;
        EventHandler::default().handle_key(key(KeyCode::Char('0')), &mut state);
        assert_eq!(state.active_page, Page::Lighting);
    }

    #[test]
    fn handle_key_digit_past_last_page_is_ignored() {
        let mut state = ApplicationState::default();
        state.active_page = Page::Lighting;
        EventHandler::default().handle_key(key(KeyCode::Char('9')), &mut state);
        assert_eq!(state.active_page, Page::Lighting);
    }

    // set_active_page

    #[test]
    fn set_active_page_out_of_range_is_ignored() {
        let mut state = ApplicationState::default();
        state.active_page = Page::Battery;
        EventHandler::default().set_active_page(99, &mut state);
        assert_eq!(state.active_page, Page::Battery);
    }

    // get_active_page_event_handler
    // Only the lighting handler has a modal, so `modal_open` tells them apart.

    #[test]
    fn active_handler_follows_active_page() {
        let handler = EventHandler::default();
        let mut state = ApplicationState::default();
        state.lighting_page_state.color_picker = Some(ColorPickerState::default());

        for (page, has_modal) in [
            (Page::Performance, false),
            (Page::Battery, false),
            (Page::Lighting, true),
        ] {
            state.active_page = page;
            assert_eq!(
                handler
                    .get_active_page_event_handler(&state)
                    .modal_open(&state),
                has_modal,
                "{page}"
            );
        }
    }

    // handle_event

    #[test]
    fn handle_event_routes_key_to_active_page() {
        let mut state = ApplicationState::default();
        state.active_page = Page::Battery;
        EventHandler::default().handle_event(
            Event::Key(key(KeyCode::Down)),
            &mut state,
            &Config::offline(),
        );
        assert_eq!(
            state.battery_page_state.active_input,
            crate::tui::state::battery::BatteryPageInput::UpperChargingLimit
        );
    }

    #[test]
    fn handle_event_open_modal_receives_keys_instead_of_app() {
        let mut state = ApplicationState::default();
        state.active_page = Page::Lighting;
        state.lighting_page_state.color_picker = Some(ColorPickerState::default());

        EventHandler::default().handle_event(
            Event::Key(key(KeyCode::Char('q'))),
            &mut state,
            &Config::offline(),
        );

        assert_eq!(state.running_state, RunningState::Running);
    }
}
