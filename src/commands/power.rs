use crate::error::Result;
use crate::hid::HidDevice;

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum PerfMode {
    Turbo,
    Perfomance,
    #[default]
    Normal,
    Quiet,
    Eco,
    EcoPlus,
}

impl PerfMode {
    fn value(&self) -> u8 {
        match self {
            PerfMode::Turbo => 0x00,
            PerfMode::Perfomance => 0x01,
            PerfMode::Normal => 0x02,
            PerfMode::Quiet => 0x03,
            PerfMode::Eco => 0x04,
            PerfMode::EcoPlus => 0x05,
        }
    }
}

pub fn set_mode(device: &HidDevice, mode: &PerfMode) -> Result<()> {
    let mut buf = [0u8; 65];
    buf[0] = 0xa0; // Report ID
    buf[1] = 0x00;
    buf[2] = 0xa0;
    buf[3] = 0x01;
    buf[4] = 0x00;
    buf[5] = 0x01;
    buf[6] = mode.value();

    device.set_feature(&buf)
}
