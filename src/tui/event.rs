use ratatui::crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent,
};
use strum::IntoEnumIterator;

use crate::tui::state::{
    ApplicationState,
    Page::{self, Battery},
    RunningState,
    battery::BatteryPageInput,
};
pub mod battery;
pub mod performance;

#[derive(Default)]
pub struct EventHandler {
    battery_page_event_handler: battery::BatteryPageEventHandler,
    performance_page_event_handler: performance::PerformancePageEventHandler,
}

impl EventHandler {
    pub fn handle_event(&self, event: Event, state: &mut ApplicationState) {
        match event {
            Event::Key(key) => {
                self.handle_key(key, state);
                self.get_active_page_event_handler(state)
                    .handle_page_event(key, state);
            }
            // After handling key events, delegate to the active page's event handler
            _ => {}
        }

        return;
    }

    fn get_active_page_event_handler(&self, state: &ApplicationState) -> &dyn PageEventHandler {
        match state.active_page {
            Page::Battery => &self.battery_page_event_handler,
            Page::Performance => &self.performance_page_event_handler,
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
}
