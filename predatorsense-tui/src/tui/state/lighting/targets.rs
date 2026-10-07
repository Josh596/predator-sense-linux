use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, FromRepr, VariantNames};

use crate::tui::state::{
    color_picker::ColorPickerState,
    fields::{ColorField, Field, OptionField, SliderField},
    lighting::{
        LightingPageInput,
        effects::{EffectDirection, LightingEffect, LightingEffectConfig},
    },
};

pub struct KeyboardState {
    pub effect: OptionField<LightingEffect>,
    pub direction: OptionField<EffectDirection>,
    pub speed: SliderField,
    pub brightness: SliderField,
    pub color: ColorField,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            effect: OptionField::new(LightingEffect::iter().collect()),
            direction: OptionField::new(EffectDirection::iter().collect()),
            speed: SliderField {
                value: 1,
                min: 1,
                max: 9,
                step: 2,
                unit: "",
                ranges: vec![],
            },
            brightness: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            color: ColorField {
                color: Color::Rgb(255, 0, 0),
            },
        }
    }
}

impl KeyboardState {
    pub fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::EffectDirection => &mut self.direction,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Effect];
        match self.effect.value().capabilities() {
            LightingEffectConfig {
                has_speed: true,
                has_direction: true,
            } => {
                inputs.push(LightingPageInput::Speed);
                inputs.push(LightingPageInput::EffectDirection);
            }
            LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            } => {
                inputs.push(LightingPageInput::Speed);
            }
            LightingEffectConfig {
                has_speed: false,
                has_direction: true,
            } => {
                inputs.push(LightingPageInput::EffectDirection);
            }
            LightingEffectConfig {
                has_speed: false,
                has_direction: false,
            } => {}
        }

        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);
        inputs
    }
}

pub struct LogoState {
    pub effect: OptionField<LightingEffect>,
    pub speed: SliderField,
    pub brightness: SliderField,
    pub color: ColorField,
}

impl Default for LogoState {
    fn default() -> Self {
        Self {
            effect: OptionField::new(vec![
                LightingEffect::Off,
                LightingEffect::Static,
                LightingEffect::Breathing,
                LightingEffect::Neon,
            ]),
            speed: SliderField {
                value: 1,
                min: 1,
                max: 9,
                step: 2,
                unit: "",
                ranges: vec![],
            },
            brightness: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            color: ColorField {
                color: Color::Rgb(255, 0, 0),
            },
        }
    }
}
impl LogoState {
    pub fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Effect];
        if self.effect.value() != LightingEffect::Static {
            inputs.push(LightingPageInput::Speed);
        }
        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);
        inputs
    }
}

pub struct TurboButtonState {
    pub brightness: SliderField,
    pub color: ColorField,
}

impl Default for TurboButtonState {
    fn default() -> Self {
        Self {
            brightness: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            color: ColorField {
                color: Color::Rgb(255, 0, 0),
            },
        }
    }
}
impl TurboButtonState {
    pub fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        vec![LightingPageInput::Brightness, LightingPageInput::Color]
    }
}
