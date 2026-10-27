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

#[cfg(test)]
mod tests {
    use super::*;

    fn press(state: &mut ApplicationState, code: KeyCode) {
        LightingPageEventHandler.handle_page_event(KeyEvent::from(code), state);
    }

    fn press_modal(state: &mut ApplicationState, code: KeyCode) {
        LightingPageEventHandler.handle_modal_event(KeyEvent::from(code), state);
    }

    fn state_with_picker() -> ApplicationState {
        let mut state = ApplicationState::default();
        state.lighting_page_state.active_input = LightingPageInput::Color;
        press(&mut state, KeyCode::Enter);
        state
    }

    // handle_page_event

    #[test]
    fn down_moves_to_next_visible_input() {
        let mut state = ApplicationState::default();
        press(&mut state, KeyCode::Down);
        assert_eq!(state.lighting_page_state.active_input, LightingPageInput::Effect);
    }

    #[test]
    fn up_moves_to_previous_visible_input() {
        let mut state = ApplicationState::default();
        state.lighting_page_state.active_input = LightingPageInput::Effect;
        press(&mut state, KeyCode::Up);
        assert_eq!(state.lighting_page_state.active_input, LightingPageInput::Target);
    }

    #[test]
    fn right_and_left_change_active_input() {
        let mut state = ApplicationState::default();
        state.lighting_page_state.active_input = LightingPageInput::Brightness;
        press(&mut state, KeyCode::Left);
        assert_eq!(state.lighting_page_state.keyboard.brightness.value, 99);
        press(&mut state, KeyCode::Right);
        assert_eq!(state.lighting_page_state.keyboard.brightness.value, 100);
    }

    #[test]
    fn enter_on_color_opens_picker_with_current_color() {
        let mut state = ApplicationState::default();
        state.lighting_page_state.keyboard.color.color = Color::Rgb(1, 2, 3);
        state.lighting_page_state.active_input = LightingPageInput::Color;

        press(&mut state, KeyCode::Enter);

        let picker = state.lighting_page_state.color_picker.as_ref().unwrap();
        assert_eq!(picker.current_color(), Color::Rgb(1, 2, 3));
    }

    #[test]
    fn enter_on_other_input_does_not_open_picker() {
        let mut state = ApplicationState::default();
        state.lighting_page_state.active_input = LightingPageInput::Brightness;
        press(&mut state, KeyCode::Enter);
        assert!(state.lighting_page_state.color_picker.is_none());
    }

    // handle_modal_event

    #[test]
    fn modal_slider_change_updates_target_color() {
        let mut state = state_with_picker();
        press_modal(&mut state, KeyCode::Down); // Preset -> R
        press_modal(&mut state, KeyCode::Left);

        let picker_color = state.lighting_page_state.color_picker.as_ref().unwrap().current_color();
        assert_eq!(state.lighting_page_state.keyboard.color.color, picker_color);
    }

    #[test]
    fn modal_preset_change_syncs_sliders_and_color() {
        let mut state = state_with_picker();
        press_modal(&mut state, KeyCode::Right); // red -> green preset
        assert_eq!(
            state.lighting_page_state.keyboard.color.color,
            Color::Rgb(0, 255, 0)
        );
    }

    #[test]
    fn modal_esc_closes_picker_and_keeps_color() {
        let mut state = state_with_picker();
        press_modal(&mut state, KeyCode::Right); // green preset
        press_modal(&mut state, KeyCode::Esc);

        assert!(state.lighting_page_state.color_picker.is_none());
        assert_eq!(
            state.lighting_page_state.keyboard.color.color,
            Color::Rgb(0, 255, 0)
        );
    }

    #[test]
    fn modal_event_without_picker_does_nothing() {
        let mut state = ApplicationState::default();
        let before = state.lighting_page_state.keyboard.color.color;
        press_modal(&mut state, KeyCode::Right);
        assert_eq!(state.lighting_page_state.keyboard.color.color, before);
    }

    // modal_open

    #[test]
    fn modal_open_reflects_picker() {
        let mut state = ApplicationState::default();
        assert!(!LightingPageEventHandler.modal_open(&state));
        state.lighting_page_state.color_picker = Some(ColorPickerState::default());
        assert!(LightingPageEventHandler.modal_open(&state));
    }
}
