use crate::error::{Error, Result};
use crate::hid::HidDevice;
use strum::IntoStaticStr;
use strum_macros::{EnumIter, FromRepr, VariantNames};

#[derive(
    Debug, Default, Eq, PartialEq, Clone, Copy, EnumIter, VariantNames, FromRepr, IntoStaticStr,
)]
#[repr(u8)]
pub enum PerfMode {
    Turbo = 0x00,
    Performance = 0x01,
    #[default]
    Normal = 0x02,
    Quiet = 0x03,
    Eco = 0x04,
    // EcoPlus,
}

impl PerfMode {
    pub fn apply(&self, device: &HidDevice) -> Result<()> {
        let mut buf = [0u8; 65];
        buf[0] = 0xa0; // Report ID
        buf[1] = 0x00;
        buf[2] = 0xa0;
        buf[3] = 0x01;
        buf[4] = 0x00;
        buf[5] = 0x01;
        buf[6] = *self as u8;

        device.set_feature(&buf)
    }

    pub fn from_system(device: &HidDevice) -> Result<Self> {
        const REPORT_ID: u8 = 0xa0;
        let request = [REPORT_ID, 0x00, 0xa0, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00];
        device.set_feature(&request)?;

        let mut buffer = [0u8; 65];
        buffer[0] = REPORT_ID;

        device.get_feature_report(&mut buffer)?;

        let raw = buffer[5];
        PerfMode::from_repr(raw).ok_or(Error::UnknownPerfMode(raw))
    }
}
