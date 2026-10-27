use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumCount, EnumIter, FromRepr, VariantNames};

use crate::commands::lighting::{Direction, Effect};

#[derive(
    Default,
    EnumIter,
    FromRepr,
    PartialEq,
    Eq,
    Clone,
    Copy,
    EnumCount,
    VariantNames,
    Display,
    Debug,
    Serialize,
    Deserialize,
)]
pub enum EffectDirection {
    #[default]
    #[strum(serialize = "Left to Right")]
    LeftToRight,
    #[strum(serialize = "Right to Left")]
    RightToLeft,
}

pub struct LightingEffectConfig {
    pub has_speed: bool,
    pub has_direction: bool,
}

#[derive(
    Default,
    EnumIter,
    FromRepr,
    PartialEq,
    Eq,
    Clone,
    Copy,
    EnumCount,
    VariantNames,
    Display,
    Serialize,
    Deserialize,
    Debug,
)]

pub enum LightingEffect {
    #[default]
    Off,
    Static,
    Breathing,
    Neon,
    Wave,
    Zoom,
    Snake,
    Disco,
    Ripple,
}

impl LightingEffect {
    pub fn capabilities(&self) -> LightingEffectConfig {
        match self {
            LightingEffect::Off => LightingEffectConfig {
                has_speed: false,
                has_direction: false,
            },
            LightingEffect::Static => LightingEffectConfig {
                has_speed: false,
                has_direction: false,
            },
            LightingEffect::Breathing => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Neon => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Wave => LightingEffectConfig {
                has_speed: true,
                has_direction: true,
            },

            LightingEffect::Zoom => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Disco => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Snake => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Ripple => LightingEffectConfig {
                has_speed: true,
                has_direction: true,
            },
        }
    }
}

impl From<LightingEffect> for Effect {
    fn from(effect: LightingEffect) -> Self {
        match effect {
            LightingEffect::Off => Effect::Off,
            LightingEffect::Static => Effect::Static,
            LightingEffect::Breathing => Effect::Breathing,
            LightingEffect::Neon => Effect::Neon,
            LightingEffect::Wave => Effect::Wave,
            LightingEffect::Ripple => Effect::Ripple,
            LightingEffect::Zoom => Effect::Zoom,
            LightingEffect::Snake => Effect::Snake,
            LightingEffect::Disco => Effect::Disco,
        }
    }
}

impl From<EffectDirection> for Direction {
    fn from(direction: EffectDirection) -> Self {
        match direction {
            EffectDirection::LeftToRight => Direction::Right,
            EffectDirection::RightToLeft => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_match_each_effect() {
        // (effect, has_speed, has_direction)
        let expected = [
            (LightingEffect::Off, false, false),
            (LightingEffect::Static, false, false),
            (LightingEffect::Breathing, true, false),
            (LightingEffect::Neon, true, false),
            (LightingEffect::Wave, true, true),
            (LightingEffect::Zoom, true, false),
            (LightingEffect::Snake, true, false),
            (LightingEffect::Disco, true, false),
            (LightingEffect::Ripple, true, true),
        ];
        for (effect, has_speed, has_direction) in expected {
            let caps = effect.capabilities();
            assert_eq!(caps.has_speed, has_speed, "{effect} has_speed");
            assert_eq!(caps.has_direction, has_direction, "{effect} has_direction");
        }
    }

    #[test]
    fn effect_maps_every_lighting_effect() {
        let expected = [
            (LightingEffect::Off, Effect::Off),
            (LightingEffect::Static, Effect::Static),
            (LightingEffect::Breathing, Effect::Breathing),
            (LightingEffect::Neon, Effect::Neon),
            (LightingEffect::Wave, Effect::Wave),
            (LightingEffect::Zoom, Effect::Zoom),
            (LightingEffect::Snake, Effect::Snake),
            (LightingEffect::Disco, Effect::Disco),
            (LightingEffect::Ripple, Effect::Ripple),
        ];
        for (lighting_effect, effect) in expected {
            assert_eq!(Effect::from(lighting_effect), effect, "{lighting_effect}");
        }
    }

    #[test]
    fn direction_maps_both_directions() {
        assert_eq!(Direction::from(EffectDirection::LeftToRight), Direction::Right);
        assert_eq!(Direction::from(EffectDirection::RightToLeft), Direction::Left);
    }
}
