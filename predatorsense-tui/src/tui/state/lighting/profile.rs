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

/// Every parameter the keyboard exposes.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct KeyboardProfile {
    brightness: u8,
    speed: u8,
    effect: LightingEffect,
    direction: EffectDirection,
    color: (u8, u8, u8),
}

impl Default for KeyboardProfile {
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


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct LogoProfile {
    brightness: u8,
    speed: u8,
    effect: LightingEffect,
    color: (u8, u8, u8),
}

impl Default for LogoProfile {
    fn default() -> Self {
        Self {
            brightness: DEFAULT_BRIGHTNESS,
            speed: DEFAULT_SPEED,
            effect: DEFAULT_EFFECT,
            color: DEFAULT_COLOR,
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct TurboProfile {
    brightness: u8,
    color: (u8, u8, u8),
}

impl Default for TurboProfile {
    fn default() -> Self {
        Self {
            brightness: DEFAULT_BRIGHTNESS,
            color: DEFAULT_COLOR,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LightingProfile {
    keyboard: KeyboardProfile,
    logo: LogoProfile,
    turbo: TurboProfile,
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

        state.keyboard.color.color = to_color(self.keyboard.color);

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
        state.logo.color.color = to_color(self.logo.color);

        // Turbo
        restore_slider(
            &mut state.turbo_button.brightness,
            self.turbo.brightness,
            "turbo.brightness",
        );
        state.turbo_button.color.color = to_color(self.turbo.color);
    }
}

impl From<&LightingPageState> for LightingProfile {
    fn from(state: &LightingPageState) -> Self {
        LightingProfile {
            keyboard: KeyboardProfile {
                brightness: state.keyboard.brightness.value as u8,
                speed: state.keyboard.speed.value as u8,
                effect: state.keyboard.effect.value(),
                direction: state.keyboard.direction.value(),
                color: rgb_from(state.keyboard.color.color),
            },
            logo: LogoProfile {
                brightness: state.logo.brightness.value as u8,
                speed: state.logo.speed.value as u8,
                effect: state.logo.effect.value(),
                color: rgb_from(state.logo.color.color),
            },
            turbo: TurboProfile {
                brightness: state.turbo_button.brightness.value as u8,
                color: rgb_from(state.turbo_button.color.color),
            },
        }
    }
}

/// Non-RGB `Color` variants (named, indexed, `Reset`) have no byte triple, so
/// they fall back to the default rather than having one invented for them.
fn rgb_from(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => DEFAULT_COLOR,
    }
}

fn to_color(rgb: (u8, u8, u8)) -> Color {
    Color::Rgb(rgb.0, rgb.1, rgb.2)
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
