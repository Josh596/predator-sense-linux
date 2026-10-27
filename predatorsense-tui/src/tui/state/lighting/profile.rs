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
struct ModeButtonProfile {
    brightness: u8,
    color: (u8, u8, u8),
}

impl Default for ModeButtonProfile {
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
    mode_button: ModeButtonProfile,
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

        // Mode
        restore_slider(
            &mut state.mode_button.brightness,
            self.mode_button.brightness,
            "mode_button.brightness",
        );
        state.mode_button.color.color = to_color(self.mode_button.color);
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
            mode_button: ModeButtonProfile {
                brightness: state.mode_button.brightness.value as u8,
                color: rgb_from(state.mode_button.color.color),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn custom_profile() -> LightingProfile {
        LightingProfile {
            keyboard: KeyboardProfile {
                brightness: 40,
                speed: 5,
                effect: LightingEffect::Wave,
                direction: EffectDirection::RightToLeft,
                color: (1, 2, 3),
            },
            logo: LogoProfile {
                brightness: 50,
                speed: 3,
                effect: LightingEffect::Breathing,
                color: (4, 5, 6),
            },
            mode_button: ModeButtonProfile {
                brightness: 60,
                color: (7, 8, 9),
            },
        }
    }

    // apply_to

    #[test]
    fn apply_to_copies_every_value_into_state() {
        let mut state = LightingPageState::default();
        custom_profile().apply_to(&mut state);

        assert_eq!(state.keyboard.brightness.value, 40);
        assert_eq!(state.keyboard.speed.value, 5);
        assert_eq!(state.keyboard.effect.value(), LightingEffect::Wave);
        assert_eq!(state.keyboard.direction.value(), EffectDirection::RightToLeft);
        assert_eq!(state.keyboard.color.color, Color::Rgb(1, 2, 3));

        assert_eq!(state.logo.brightness.value, 50);
        assert_eq!(state.logo.speed.value, 3);
        assert_eq!(state.logo.effect.value(), LightingEffect::Breathing);
        assert_eq!(state.logo.color.color, Color::Rgb(4, 5, 6));

        assert_eq!(state.mode_button.brightness.value, 60);
        assert_eq!(state.mode_button.color.color, Color::Rgb(7, 8, 9));
    }

    #[test]
    fn apply_to_clamps_out_of_range_values() {
        let mut profile = custom_profile();
        profile.keyboard.speed = 20;
        profile.logo.brightness = 200;

        let mut state = LightingPageState::default();
        profile.apply_to(&mut state);

        assert_eq!(state.keyboard.speed.value, 9);
        assert_eq!(state.logo.brightness.value, 100);
    }

    #[test]
    fn apply_to_keeps_default_when_effect_not_available() {
        let mut profile = custom_profile();
        profile.logo.effect = LightingEffect::Wave; // logo has no Wave

        let mut state = LightingPageState::default();
        profile.apply_to(&mut state);

        assert_eq!(state.logo.effect.value(), LightingEffect::Off);
    }

    // From<&LightingPageState>

    #[test]
    fn from_state_reads_every_value() {
        let mut state = LightingPageState::default();
        state.keyboard.brightness.set_value(40);
        state.logo.speed.set_value(3);
        state.mode_button.color.color = Color::Rgb(7, 8, 9);

        let profile = LightingProfile::from(&state);

        assert_eq!(profile.keyboard.brightness, 40);
        assert_eq!(profile.logo.speed, 3);
        assert_eq!(profile.mode_button.color, (7, 8, 9));
    }

    // Round trips

    #[test]
    fn state_round_trip_keeps_profile() {
        let profile = custom_profile();
        let state = LightingPageState::new(&profile);
        assert_eq!(LightingProfile::from(&state), profile);
    }

    #[test]
    fn toml_round_trip_keeps_profile() {
        let profile = custom_profile();
        let text = toml::to_string(&profile).unwrap();
        let parsed: LightingProfile = toml::from_str(&text).unwrap();
        assert_eq!(parsed, profile);
    }

    #[test]
    fn partial_toml_fills_missing_fields_with_defaults() {
        let parsed: LightingProfile = toml::from_str("[keyboard]\nbrightness = 40\n").unwrap();

        let mut expected = LightingProfile::default();
        expected.keyboard.brightness = 40;
        assert_eq!(parsed, expected);
    }

    // Helpers

    #[test]
    fn rgb_from_non_rgb_color_returns_default() {
        assert_eq!(rgb_from(Color::Red), DEFAULT_COLOR);
    }
}
