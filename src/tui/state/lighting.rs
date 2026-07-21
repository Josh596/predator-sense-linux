use ratatui::style::Color;
use strum::{EnumCount, VariantNames};
use strum_macros::{EnumCount, EnumIter, FromRepr, VariantNames};

use crate::tui::state::fields::{ColorField, Field, OptionField, SliderField};

#[derive(Default, PartialEq, Eq, Clone, Copy, EnumCount, EnumIter, FromRepr, Hash)]
pub enum LightingPageInput {
    #[default]
    Target,
    Effect,
    Brightness,
    Speed,
    EffectDirection,
    Color,
}

#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames)]
pub enum LightingMode {
    #[default]
    Static,
    Dynamic,
}
#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames)]
pub enum Target {
    #[default]
    Keyboard,
    Logo,
    TurboButton,
}

#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames)]
pub enum EffectDirection {
    #[default]
    LeftToRight,
    RightToLeft,
}

#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames)]
pub enum LightingEffect {
    #[default]
    Static,
    Breathing,
    Neon,
    Wave,
    Shifting,
    Zoom,
}
pub struct LightingPageState {
    pub active_input: LightingPageInput,
    pub effect_input: OptionField,
    pub brightness_input: SliderField,
    pub speed_input: SliderField,
    pub effect_direction_input: OptionField,
    pub color_input: ColorField,
    pub target_input: OptionField,
}

impl LightingPageState {
    pub fn get_active_input_mut(&mut self) -> &mut dyn Field {
        match self.active_input {
            LightingPageInput::Effect => &mut self.effect_input,
            LightingPageInput::Brightness => &mut self.brightness_input,
            LightingPageInput::Speed => &mut self.speed_input,
            LightingPageInput::EffectDirection => &mut self.effect_direction_input,
            LightingPageInput::Color => &mut self.color_input,
            LightingPageInput::Target => &mut self.target_input,
        }
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Target];

        if self.target_input.selected_option_index == Target::Keyboard as usize {
            inputs.push(LightingPageInput::Effect);
        }

        if self.effect_input.selected_option_index != LightingEffect::Static as usize {
            inputs.push(LightingPageInput::Speed);
            inputs.push(LightingPageInput::EffectDirection);
        }

        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);

        inputs
    }

    pub fn next_input(&self) -> LightingPageInput {
        let visible_inputs = self.visible_inputs();
        let current_index = visible_inputs
            .iter()
            .position(|&input| input == self.active_input)
            .unwrap_or(0);
        let next_index = (current_index + 1).min(visible_inputs.len() - 1);
        visible_inputs[next_index]
    }

    pub fn prev_input(&self) -> LightingPageInput {
        let visible_inputs = self.visible_inputs();
        let current_index = visible_inputs
            .iter()
            .position(|&input| input == self.active_input)
            .unwrap_or(0);
        let prev_index = current_index.saturating_sub(1);
        visible_inputs[prev_index]
    }
}

impl Default for LightingPageState {
    fn default() -> Self {
        Self {
            active_input: LightingPageInput::default(),
            effect_input: OptionField {
                options: LightingEffect::VARIANTS
                    .iter()
                    .map(|variant| String::from(*variant))
                    .collect(),
                selected_option_index: LightingEffect::default() as usize,
            },
            brightness_input: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            speed_input: SliderField {
                value: 1,
                min: 1,
                max: 9,
                step: 2,
                unit: "",
                ranges: vec![],
            },
            effect_direction_input: OptionField {
                options: EffectDirection::VARIANTS
                    .iter()
                    .map(|variant| String::from(*variant))
                    .collect(),
                selected_option_index: EffectDirection::default() as usize,
            },
            color_input: ColorField {
                color: Color::Rgb(255, 0, 0),
                popup_open: false,
            },
            target_input: OptionField {
                options: Target::VARIANTS
                    .iter()
                    .map(|variant| String::from(*variant))
                    .collect(),
                selected_option_index: Target::default() as usize,
            },
        }
    }
}
