use serde::{Deserialize, Serialize};

use crate::{
    commands::lighting::{Direction, Effect, LightingCommand, Rgb, Speed, Target, Zone},
    effects::{EffectDirection, LightingEffect},
};

pub const DEFAULT_COLOR: (u8, u8, u8) = (0, 255, 0);
const DEFAULT_DIRECTION: EffectDirection = EffectDirection::LeftToRight;
const DEFAULT_BRIGHTNESS: u8 = 100;
const DEFAULT_SPEED: u8 = 1;
const DEFAULT_EFFECT: LightingEffect = LightingEffect::Off;

/// Every parameter the keyboard exposes.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct KeyboardProfile {
    pub brightness: u8,
    pub speed: u8,
    pub effect: LightingEffect,
    pub direction: EffectDirection,
    pub color: (u8, u8, u8),
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
pub struct LogoProfile {
    pub brightness: u8,
    pub speed: u8,
    pub effect: LightingEffect,
    pub color: (u8, u8, u8),
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
pub struct ModeButtonProfile {
    pub brightness: u8,
    pub speed: u8,
    pub effect: LightingEffect,
    pub color: (u8, u8, u8),
}

impl Default for ModeButtonProfile {
    fn default() -> Self {
        Self {
            brightness: DEFAULT_BRIGHTNESS,
            speed: DEFAULT_SPEED,
            effect: DEFAULT_EFFECT,
            color: DEFAULT_COLOR,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LightingProfile {
    pub keyboard: KeyboardProfile,
    pub logo: LogoProfile,
    pub mode_button: ModeButtonProfile,
}

impl LightingProfile {
    /// One command per target, in the order the device expects them.
    pub fn commands(&self) -> Vec<LightingCommand> {
        vec![
            LightingCommand {
                target: Target::Keyboard,
                effect: Effect::from(self.keyboard.effect),
                brightness: self.keyboard.brightness,
                speed: Speed::new(self.keyboard.speed),
                direction: Direction::from(self.keyboard.direction),
                color: Rgb::from(self.keyboard.color),
                zone: Zone::All,
            },
            // The logo and mode button have no direction and no zones.
            LightingCommand {
                target: Target::BackLogo,
                effect: Effect::from(self.logo.effect),
                brightness: self.logo.brightness,
                speed: Speed::new(self.logo.speed),
                direction: Direction::None,
                color: Rgb::from(self.logo.color),
                zone: Zone::None,
            },
            LightingCommand {
                target: Target::PerformanceModeButton,
                effect: Effect::from(self.mode_button.effect),
                brightness: self.mode_button.brightness,
                speed: Speed::new(self.mode_button.speed),
                direction: Direction::None,
                color: Rgb::from(self.mode_button.color),
                zone: Zone::None,
            },
        ]
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
                speed: 2,
                effect: LightingEffect::Neon,
                color: (7, 8, 9),
            },
        }
    }

    // commands

    #[test]
    fn commands_cover_each_target_once() {
        let targets: Vec<Target> = custom_profile()
            .commands()
            .iter()
            .map(|c| c.target)
            .collect();
        assert_eq!(
            targets,
            vec![
                Target::Keyboard,
                Target::BackLogo,
                Target::PerformanceModeButton
            ]
        );
    }

    #[test]
    fn commands_use_expected_zones_and_directions() {
        let commands = custom_profile().commands();
        let zones: Vec<Zone> = commands.iter().map(|c| c.zone).collect();
        let directions: Vec<Direction> = commands.iter().map(|c| c.direction).collect();
        assert_eq!(zones, vec![Zone::All, Zone::None, Zone::None]);
        assert_eq!(
            directions,
            vec![Direction::Left, Direction::None, Direction::None]
        );
    }

    #[test]
    fn commands_copy_every_value() {
        let commands = custom_profile().commands();

        assert_eq!(commands[0].effect, Effect::Wave);
        assert_eq!(commands[0].brightness, 40);
        assert_eq!(commands[0].speed, Speed::new(5));
        assert_eq!(commands[0].color, Rgb { r: 1, g: 2, b: 3 });

        assert_eq!(commands[1].effect, Effect::Breathing);
        assert_eq!(commands[1].brightness, 50);
        assert_eq!(commands[1].speed, Speed::new(3));
        assert_eq!(commands[1].color, Rgb { r: 4, g: 5, b: 6 });

        assert_eq!(commands[2].effect, Effect::Neon);
        assert_eq!(commands[2].brightness, 60);
        assert_eq!(commands[2].speed, Speed::new(2));
        assert_eq!(commands[2].color, Rgb { r: 7, g: 8, b: 9 });
    }

    // serde

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
}
