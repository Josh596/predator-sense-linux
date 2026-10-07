use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
};

use crate::tui::{
    event::PageEventHandler,
    state::{
        ApplicationState,
        color_picker::{ColorPickerInput, ColorPickerState},
        lighting::LightingPageInput,
    },
};
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

                if state.lighting_page_state.active_input == LightingPageInput::Color {
                    state.lighting_page_state.color_picker = Some(ColorPickerState::from(
                        state.lighting_page_state.active_color_field_mut().color,
                    ));
                }
            }
            _ => {}
        }
    }

    fn handle_modal_event(&self, key: KeyEvent, state: &mut ApplicationState) {
        if !state.lighting_page_state.color_picker.is_some() {
            return;
        }
        let color_picker = state.lighting_page_state.color_picker.as_mut().unwrap();
        // let active_input =.color_picker.active_input;
        match key.code {
            KeyCode::Up => {
                color_picker.active_input = color_picker.prev_input();
            }
            KeyCode::Down => {
                color_picker.active_input = color_picker.next_input();
            }
            KeyCode::Left => {
                color_picker.get_active_input_mut().decrement();

                if color_picker.active_input == ColorPickerInput::Preset {
                    color_picker.sync();
                }
            }
            KeyCode::Right => {
                color_picker.get_active_input_mut().increment();

                if color_picker.active_input == ColorPickerInput::Preset {
                    color_picker.sync();
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                color_picker.get_active_input_mut().toggle();
            }

            _ => {}
        }

        // if color_picker.active_input == ColorPickerInput::Preset {
        //     color_picker.sync();
        // }

        state.lighting_page_state.active_color_field_mut().color = Color::Rgb(
            color_picker.r_color.value as u8,
            color_picker.g_color.value as u8,
            color_picker.b_color.value as u8,
        );

        if key.code == KeyCode::Esc {
            state.lighting_page_state.color_picker = None;
        }
    }

    fn modal_open(&self, state: &ApplicationState) -> bool {
        state.lighting_page_state.color_picker.is_some()
    }
}
