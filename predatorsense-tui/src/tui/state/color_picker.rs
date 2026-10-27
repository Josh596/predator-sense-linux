use ratatui::style::Color;
use strum::{EnumCount, FromRepr};

use crate::tui::state::fields::{Field, OptionField, SliderField};

pub enum ColorChannel {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumCount)]
pub enum ColorPickerInput {
    Preset,
    R,
    G,
    B,
}

pub struct ColorPickerState {
    // active_row:
    pub active_input: ColorPickerInput,
    pub preset_input: OptionField<Preset>,
    pub r_color: SliderField,
    pub g_color: SliderField,
    pub b_color: SliderField,
}

impl ColorPickerState {
    pub fn next_input(&self) -> ColorPickerInput {
        let i = (self.active_input as usize + 1).min(ColorPickerInput::COUNT - 1);
        ColorPickerInput::from_repr(i).unwrap()
    }

    pub fn prev_input(&self) -> ColorPickerInput {
        ColorPickerInput::from_repr((self.active_input as usize).saturating_sub(1)).unwrap()
    }

    pub fn get_active_input_mut(&mut self) -> &mut dyn Field {
        match self.active_input {
            ColorPickerInput::Preset => &mut self.preset_input,
            ColorPickerInput::R => &mut self.r_color,
            ColorPickerInput::G => &mut self.g_color,
            ColorPickerInput::B => &mut self.b_color,
        }
    }
    pub fn sync(&mut self) {
        if let Color::Rgb(r, g, b) = self.preset_input.value().0 {
            self.r_color.set_value(r as usize);
            self.b_color.set_value(b as usize);
            self.g_color.set_value(g as usize);
        }
    }

    pub fn current_color(&self) -> Color {
        Color::Rgb(
            self.r_color.value as u8,
            self.g_color.value as u8,
            self.b_color.value as u8,
        )
    }
}

impl Default for ColorPickerState {
    fn default() -> Self {
        Self {
            active_input: ColorPickerInput::Preset,
            preset_input: OptionField::new(vec![
                Preset(Color::Rgb(255, 0, 0)),     // red
                Preset(Color::Rgb(0, 255, 0)),     // green
                Preset(Color::Rgb(0, 0, 255)),     // blue
                Preset(Color::Rgb(255, 255, 0)),   // yellow
                Preset(Color::Rgb(255, 0, 255)),   // magenta
                Preset(Color::Rgb(0, 255, 255)),   // cyan
                Preset(Color::Rgb(255, 255, 255)), // white
            ]),
            r_color: SliderField {
                value: 255,
                min: 0,
                max: 255,
                step: 1,
                unit: "",
                ranges: vec![],
            },
            g_color: SliderField {
                value: 0,
                min: 0,
                max: 255,
                step: 1,
                unit: "",
                ranges: vec![],
            },
            b_color: SliderField {
                value: 0,
                min: 0,
                max: 255,
                step: 1,
                unit: "",
                ranges: vec![],
            },
        }
    }
}

impl From<Color> for ColorPickerState {
    fn from(color: Color) -> Self {
        let mut state = ColorPickerState::default();
        if let Color::Rgb(r, g, b) = color {
            state.r_color.set_value(r as usize);
            state.g_color.set_value(g as usize);
            state.b_color.set_value(b as usize);
        }
        state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preset(pub Color);

#[cfg(test)]
mod tests {
    use super::*;

    fn picker_at(active_input: ColorPickerInput) -> ColorPickerState {
        let mut state = ColorPickerState::from(Color::Rgb(10, 20, 30));
        state.active_input = active_input;
        state
    }

    fn sliders(state: &ColorPickerState) -> (usize, usize, usize) {
        (state.r_color.value, state.g_color.value, state.b_color.value)
    }

    // next_input / prev_input

    #[test]
    fn next_input_moves_to_next() {
        assert_eq!(picker_at(ColorPickerInput::Preset).next_input(), ColorPickerInput::R);
    }

    #[test]
    fn next_input_stops_at_last() {
        assert_eq!(picker_at(ColorPickerInput::B).next_input(), ColorPickerInput::B);
    }

    #[test]
    fn prev_input_moves_to_previous() {
        assert_eq!(picker_at(ColorPickerInput::G).prev_input(), ColorPickerInput::R);
    }

    #[test]
    fn prev_input_stops_at_first() {
        assert_eq!(
            picker_at(ColorPickerInput::Preset).prev_input(),
            ColorPickerInput::Preset
        );
    }

    // get_active_input_mut

    #[test]
    fn active_input_preset_returns_preset_field() {
        let mut state = picker_at(ColorPickerInput::Preset);
        state.get_active_input_mut().increment();
        assert_eq!(state.preset_input.selected_option_index, 1);
        assert_eq!(sliders(&state), (10, 20, 30));
    }

    #[test]
    fn active_input_r_returns_red_slider() {
        let mut state = picker_at(ColorPickerInput::R);
        state.get_active_input_mut().increment();
        assert_eq!(sliders(&state), (11, 20, 30));
    }

    #[test]
    fn active_input_g_returns_green_slider() {
        let mut state = picker_at(ColorPickerInput::G);
        state.get_active_input_mut().increment();
        assert_eq!(sliders(&state), (10, 21, 30));
    }

    #[test]
    fn active_input_b_returns_blue_slider() {
        let mut state = picker_at(ColorPickerInput::B);
        state.get_active_input_mut().increment();
        assert_eq!(sliders(&state), (10, 20, 31));
    }

    // sync

    #[test]
    fn sync_copies_preset_into_sliders() {
        let mut state = picker_at(ColorPickerInput::Preset);
        state.preset_input.selected_option_index = 1; // green
        state.sync();
        assert_eq!(sliders(&state), (0, 255, 0));
    }

    // current_color

    #[test]
    fn current_color_is_built_from_sliders() {
        let state = picker_at(ColorPickerInput::Preset);
        assert_eq!(state.current_color(), Color::Rgb(10, 20, 30));
    }

    // default

    #[test]
    fn default_starts_on_red_preset() {
        let state = ColorPickerState::default();
        assert_eq!(state.active_input, ColorPickerInput::Preset);
        assert_eq!(state.preset_input.value(), Preset(Color::Rgb(255, 0, 0)));
        assert_eq!(sliders(&state), (255, 0, 0));
    }

    // From<Color>

    #[test]
    fn from_rgb_color_sets_sliders() {
        let state = ColorPickerState::from(Color::Rgb(1, 2, 3));
        assert_eq!(sliders(&state), (1, 2, 3));
    }

    #[test]
    fn from_non_rgb_color_keeps_default_sliders() {
        let state = ColorPickerState::from(Color::Red);
        assert_eq!(sliders(&state), (255, 0, 0));
    }
}
