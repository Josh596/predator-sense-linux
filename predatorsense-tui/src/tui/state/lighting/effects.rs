use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumCount, EnumIter, FromRepr, VariantNames};


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
