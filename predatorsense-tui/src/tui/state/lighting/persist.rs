use predatorsense::profile::{
    DEFAULT_COLOR, KeyboardProfile, LightingProfile, LogoProfile, ModeButtonProfile,
};
use ratatui::style::Color;

use crate::tui::state::{fields::SliderField, lighting::LightingPageState};

impl LightingPageState {
    pub fn apply_profile(&mut self, profile: &LightingProfile) {
        restore_slider(
            &mut self.keyboard.brightness,
            profile.keyboard.brightness,
            "keyboard.brightness",
        );
        restore_slider(
            &mut self.keyboard.speed,
            profile.keyboard.speed,
            "keyboard.speed",
        );
        if !self.keyboard.effect.select_value(profile.keyboard.effect) {
            log::warn!(
                "keyboard: effect {} not available, keeping default",
                profile.keyboard.effect
            );
        }
        if !self
            .keyboard
            .direction
            .select_value(profile.keyboard.direction)
        {
            log::warn!(
                "keyboard: direction {} not available, keeping default",
                profile.keyboard.direction
            );
        }

        self.keyboard.color.color = to_color(profile.keyboard.color);

        // Logo
        restore_slider(
            &mut self.logo.brightness,
            profile.logo.brightness,
            "logo.brightness",
        );
        restore_slider(&mut self.logo.speed, profile.logo.speed, "logo.speed");
        if !self.logo.effect.select_value(profile.logo.effect) {
            log::warn!(
                "logo: effect {} not available, keeping default",
                profile.logo.effect
            );
        }
        self.logo.color.color = to_color(profile.logo.color);

        // Mode
        restore_slider(
            &mut self.mode_button.brightness,
            profile.mode_button.brightness,
            "mode_button.brightness",
        );
        restore_slider(
            &mut self.mode_button.speed,
            profile.mode_button.speed,
            "mode_button.speed",
        );
        if !self.mode_button.effect.select_value(profile.mode_button.effect) {
            log::warn!(
                "mode_button: effect {} not available, keeping default",
                profile.mode_button.effect
            );
        }
        self.mode_button.color.color = to_color(profile.mode_button.color);
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
                speed: state.mode_button.speed.value as u8,
                effect: state.mode_button.effect.value(),
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
    use predatorsense::effects::{EffectDirection, LightingEffect};

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
                speed: 2,
                effect: LightingEffect::Neon,
                color: (7, 8, 9),
            },
        }
    }

    // apply_profile

    #[test]
    fn apply_profile_copies_every_value_into_state() {
        let mut state = LightingPageState::default();
        state.apply_profile(&custom_profile());

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
        assert_eq!(state.mode_button.speed.value, 2);
        assert_eq!(state.mode_button.effect.value(), LightingEffect::Neon);
        assert_eq!(state.mode_button.color.color, Color::Rgb(7, 8, 9));
    }

    #[test]
    fn apply_profile_clamps_out_of_range_values() {
        let mut profile = custom_profile();
        profile.keyboard.speed = 20;
        profile.logo.brightness = 200;

        let mut state = LightingPageState::default();
        state.apply_profile(&profile);

        assert_eq!(state.keyboard.speed.value, 9);
        assert_eq!(state.logo.brightness.value, 100);
    }

    #[test]
    fn apply_profile_keeps_default_when_effect_not_available() {
        let mut profile = custom_profile();
        profile.logo.effect = LightingEffect::Wave; // logo has no Wave

        let mut state = LightingPageState::default();
        state.apply_profile(&profile);

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

    // Helpers

    #[test]
    fn rgb_from_non_rgb_color_returns_default() {
        assert_eq!(rgb_from(Color::Red), DEFAULT_COLOR);
    }
}
