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
}
impl Zone {
    fn value(&self) -> u8 {
        match self {
            Zone::All => 0x0f,
            Zone::One => 0x01,
            Zone::Two => 0x02,
            Zone::Three => 0x04,
            Zone::Four => 0x08, // bitmask, NOT sequential
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
    PowerProfileButton,
}

impl Target {
    fn value(&self) -> u8 {
        match self {
            Target::Keyboard => 0x21,
            Target::BackLogo => 0x83,
            Target::PowerProfileButton => 0x65,
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

pub fn apply(device: &HidDevice, cmd: &LightingCommand) -> Result<()> {
    let buf = [
        0xa4, // report ID
        cmd.target.value(),
        cmd.effect.value(),
        cmd.brightness.min(100), // 0–100
        cmd.speed.value(),
        cmd.direction.value(),
        cmd.color.r,
        cmd.color.g,
        cmd.color.b,
        cmd.zone.value(),
        0x00,
    ];
    device.set_feature(&buf)
}
