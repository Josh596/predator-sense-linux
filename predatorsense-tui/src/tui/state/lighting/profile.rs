use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::tui::state::{
    fields::SliderField,
    lighting::{
        LightingPageState,
        effects::{EffectDirection, LightingEffect},
    },
};

const DEFAULT_COLOR: (u8, u8, u8) = (0, 255, 0);
const DEFAULT_DIRECTION: EffectDirection = EffectDirection::LeftToRight;
const DEFAULT_BRIGHTNESS: u8 = 100;
const DEFAULT_SPEED: u8 = 1;
const DEFAULT_EFFECT: LightingEffect = LightingEffect::Off;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct LightingSettings {
    brightness: u8,
    speed: u8,
    effect: LightingEffect,
    direction: EffectDirection,
    color: (u8, u8, u8),
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            brightness: DEFAULT_BRIGHTNESS,
            speed: DEFAULT_SPEED,
            effect: DEFAULT_EFFECT,
            direction: DEFAULT_DIRECTION,
            color: DEFAULT_COLOR,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LightingProfile {
    keyboard: LightingSettings,
    logo: LightingSettings,
    turbo: LightingSettings,
}

impl LightingProfile {
    pub fn apply_to(&self, state: &mut LightingPageState) {
        restore_slider(
            &mut state.keyboard.brightness,
            self.keyboard.brightness,
            "keyboard.brightness",
        );
        restore_slider(
            &mut state.keyboard.speed,
            self.keyboard.speed,
            "keyboard.speed",
        );
        if !state.keyboard.effect.select_value(self.keyboard.effect) {
            log::warn!(
                "keyboard: effect {} not available, keeping default",
                self.keyboard.effect
            );
        }
        if !state
            .keyboard
            .direction
            .select_value(self.keyboard.direction)
        {
            log::warn!(
                "keyboard: direction {} not available, keeping default",
                self.keyboard.direction
            );
        }

        state.keyboard.color.color = Color::Rgb(
            self.keyboard.color.0,
            self.keyboard.color.1,
            self.keyboard.color.2,
        );

        // Logo
        restore_slider(
            &mut state.logo.brightness,
            self.logo.brightness,
            "logo.brightness",
        );
        restore_slider(&mut state.logo.speed, self.logo.speed, "logo.speed");
        if !state.logo.effect.select_value(self.logo.effect) {
            log::warn!(
                "logo: effect {} not available, keeping default",
                self.logo.effect
            );
        }
        state.logo.color.color =
            Color::Rgb(self.logo.color.0, self.logo.color.1, self.logo.color.2);

        // Turbo
        restore_slider(
            &mut state.turbo_button.brightness,
            self.turbo.brightness,
            "turbo.brightness",
        );
        state.turbo_button.color.color =
            Color::Rgb(self.turbo.color.0, self.turbo.color.1, self.turbo.color.2);
    }
}

impl From<&LightingPageState> for LightingProfile {
    fn from(state: &LightingPageState) -> Self {
        // keyboard settings
        let keyboard = LightingSettings {
            brightness: state.keyboard.brightness.value as u8,
            speed: state.keyboard.speed.value as u8,
            effect: state.keyboard.effect.value(),
            color: match state.keyboard.color.color {
                Color::Rgb(r, g, b) => {
                    (r, g, b)
                    // use r, g, b (u8)
                }
                _ => DEFAULT_COLOR,
            },
            direction: state.keyboard.direction.value(), // color: (s)
        };

        let logo = LightingSettings {
            brightness: state.logo.brightness.value as u8,
            speed: state.logo.speed.value as u8,
            effect: state.logo.effect.value(),
            color: match state.logo.color.color {
                Color::Rgb(r, g, b) => {
                    (r, g, b)
                    // use r, g, b (u8)
                }
                _ => DEFAULT_COLOR,
            },
            direction: DEFAULT_DIRECTION, // color: (s)
        };

        let turbo = LightingSettings {
            brightness: state.turbo_button.brightness.value as u8,
            speed: DEFAULT_SPEED,
            effect: DEFAULT_EFFECT,
            color: match state.turbo_button.color.color {
                Color::Rgb(r, g, b) => {
                    (r, g, b)
                    // use r, g, b (u8)
                }
                _ => DEFAULT_COLOR,
            },
            direction: DEFAULT_DIRECTION, // color: (s)
        };

        LightingProfile {
            keyboard,
            logo,
            turbo,
        }
    }
}

fn restore_slider(field: &mut SliderField, value: u8, what: &str) {
    if !field.set_value(value as usize) {
        log::warn!(
            "{what}: {value} out of range {}..={}, clamped to {}",
            field.min,
            field.max,
            field.value
        )
    }
}
