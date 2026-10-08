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

const PLACEHOLDER_COLOR: Color = Color::Rgb(255, 255, 255);
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
            speed: SliderField::new(0, 9, 1, ""),
            brightness: SliderField::new(0, 100, 1, "%"),
            color: ColorField {
                color: PLACEHOLDER_COLOR,
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
        let caps = self.effect.value().capabilities();

        let mut inputs = vec![LightingPageInput::Effect];
        if caps.has_speed {
            inputs.push(LightingPageInput::Speed);
        }
        if caps.has_direction {
            inputs.push(LightingPageInput::EffectDirection);
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
            speed: SliderField::new(0, 9, 1, ""),
            brightness: SliderField::new(0, 100, 1, "%"),
            color: ColorField {
                color: PLACEHOLDER_COLOR,
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
        if self.effect.value().capabilities().has_speed {
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
            brightness: SliderField::new(0, 100, 1, "%"),
            color: ColorField {
                color: PLACEHOLDER_COLOR,
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
