use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::tui::{event::PageEventHandler, state::ApplicationState};
#[derive(Default)]
pub struct LightingPageEventHandler;

impl LightingPageEventHandler {
    fn modify_active_input(&self, _state: &mut ApplicationState, _increase: bool) {}
}

impl PageEventHandler for LightingPageEventHandler {
    fn handle_page_event(&self, key: KeyEvent, state: &mut ApplicationState) {
        // s|d and arrow to change static_dynamic mode
        match key.code {
            KeyCode::Up => {
                state.lighting_page_state.active_input = state.lighting_page_state.prev_input();
            }
            KeyCode::Down => {
                state.lighting_page_state.active_input = state.lighting_page_state.next_input();
            }
            KeyCode::Left => {
                state.lighting_page_state.get_active_input_mut().decrement();
            }
            KeyCode::Right => {
                state.lighting_page_state.get_active_input_mut().increment();
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                state.lighting_page_state.get_active_input_mut().toggle();
            }
            _ => {}
        }
    }
}
