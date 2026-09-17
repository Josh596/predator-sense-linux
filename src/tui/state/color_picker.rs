use ratatui::style::Color;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use crate::tui::state::{
    color_picker::ColorPickerInput::R,
    fields::{Field, OptionField, SliderField},
};

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
            self.r_color.value = r as usize;
            self.g_color.value = g as usize;
            self.b_color.value = b as usize;
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
            state.r_color.value = r as usize;
            state.g_color.value = g as usize;
            state.b_color.value = b as usize;
        }
        state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preset(pub Color);
