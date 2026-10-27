use strum_macros::{Display, EnumIter, VariantNames};

use crate::error::Result;
use crate::hid::HidDevice;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Speed(u8);

impl Speed {
    pub fn new(value: u8) -> Self {
        Speed(value.min(Speed::max() as u8))
    }
    pub fn max() -> usize {
        return 9;
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Display, EnumIter)]
pub enum Zone {
    #[default]
    All,
    #[strum(to_string = "Z1")]
    One,
    #[strum(to_string = "Z2")]
    Two,
    #[strum(to_string = "Z3")]
    Three,
    #[strum(to_string = "Z4")]
    Four,
    None,
}
impl Zone {
    fn value(&self) -> u8 {
        match self {
            Zone::All => 0x0f,
            Zone::One => 0x01,
            Zone::Two => 0x02,
            Zone::Three => 0x04,
            Zone::Four => 0x08, // bitmask, NOT sequential
            Zone::None => 0x00,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, EnumIter, Display)]
pub enum Direction {
    #[default]
    None,
    Right,
    Left,
}

impl Direction {
    fn value(&self) -> u8 {
        match self {
            Direction::None => 0x00,
            Direction::Right => 0x01,
            Direction::Left => 0x02,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub enum Effect {
    Off,
    #[default]
    Static,
    Breathing,
    Neon,
    PowerProfile,
    Wave,
    Ripple,
    Zoom,
    Snake,
    Disco,
    Shifting,
}

impl Effect {
    pub fn value(&self) -> u8 {
        match self {
            Effect::Off => 0x01,
            Effect::Static => 0x02,
            Effect::Breathing => 0x04,
            Effect::Neon => 0x05,
            Effect::PowerProfile => 0x06,
            Effect::Wave => 0x07,
            Effect::Ripple => 0x08,
            Effect::Zoom => 0x09,
            Effect::Snake => 0x0a,
            Effect::Disco => 0x0b,
            Effect::Shifting => 0xff,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, EnumIter, VariantNames, Default, Display)]
pub enum Target {
    #[default]
    Keyboard,
    BackLogo,
    PerformanceModeButton,
}

impl Target {
    fn value(&self) -> u8 {
        match self {
            Target::Keyboard => 0x21,
            Target::BackLogo => 0x83,
            Target::PerformanceModeButton => 0x65,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct LightingCommand {
    pub target: Target,
    pub effect: Effect,
    pub brightness: u8,
    pub speed: Speed,
    pub direction: Direction,
    pub color: Rgb,
    pub zone: Zone,
}

impl LightingCommand {
    pub fn apply(&self, device: &HidDevice) -> Result<()> {
        let buf = [
            0xa4, // report ID
            self.target.value(),
            self.effect.value(),
            self.brightness.min(100), // 0–100
            self.speed.value(),
            self.direction.value(),
            self.color.r,
            self.color.g,
            self.color.b,
            self.zone.value(),
            0x00,
        ];
        device.set_feature(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Speed

    #[test]
    fn speed_is_clamped_to_max() {
        let speed = Speed::new(10);
        assert_eq!(speed.value(), 9);
    }

    #[test]
    fn speed_within_range_is_kept() {
        assert_eq!(Speed::new(5).value(), 5);
    }

    // Zone

    #[test]
    fn zone_values_match_protocol() {
        assert_eq!(Zone::All.value(), 0x0f);
        assert_eq!(Zone::One.value(), 0x01);
        assert_eq!(Zone::Two.value(), 0x02);
        assert_eq!(Zone::Three.value(), 0x04);
        assert_eq!(Zone::Four.value(), 0x08);
        assert_eq!(Zone::None.value(), 0x00);
    }

    // Direction

    #[test]
    fn direction_values_match_protocol() {
        assert_eq!(Direction::None.value(), 0x00);
        assert_eq!(Direction::Right.value(), 0x01);
        assert_eq!(Direction::Left.value(), 0x02);
    }

    // Effect

    #[test]
    fn effect_values_match_protocol() {
        assert_eq!(Effect::Off.value(), 0x01);
        assert_eq!(Effect::Static.value(), 0x02);
        assert_eq!(Effect::Breathing.value(), 0x04);
        assert_eq!(Effect::Neon.value(), 0x05);
        assert_eq!(Effect::PowerProfile.value(), 0x06);
        assert_eq!(Effect::Wave.value(), 0x07);
        assert_eq!(Effect::Ripple.value(), 0x08);
        assert_eq!(Effect::Zoom.value(), 0x09);
        assert_eq!(Effect::Snake.value(), 0x0a);
        assert_eq!(Effect::Disco.value(), 0x0b);
        assert_eq!(Effect::Shifting.value(), 0xff);
    }

    // Target

    #[test]
    fn target_values_match_protocol() {
        assert_eq!(Target::Keyboard.value(), 0x21);
        assert_eq!(Target::BackLogo.value(), 0x83);
        assert_eq!(Target::PerformanceModeButton.value(), 0x65);
    }
}
