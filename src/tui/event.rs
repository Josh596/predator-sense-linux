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

                    return;
                }
                self.handle_key(key, state);

                self.get_active_page_event_handler(state)
                    .handle_page_event(key, state);
            }
            // After handling key events, delegate to the active page's event handler
            _ => {}
        }
        let new_simple_state = Applied::desired(state);

        // TODO: HANDLE THE ERROR
        services::execute(old_simple_state, new_simple_state, config);

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
            KeyCode::Char(c) if c.is_ascii_digit() => {
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
    fn modal_open(&self, state: &ApplicationState) -> bool {
        false
    }
    fn handle_modal_event(&self, key: KeyEvent, state: &mut ApplicationState) {}
}
