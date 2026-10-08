use crate::error::Error;
use crate::error::Result;
use crate::hid::HidDevice;

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct ChargingLimit {
    pub enabled: bool,
    pub upper: u8,
    pub lower: u8,
}

impl ChargingLimit {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            upper: 0,
            lower: 0,
        }
    }

    pub fn enabled(upper: u8, lower: u8) -> Result<Self> {
        if upper > 100 || lower > 100 {
            return Err(Error::InvalidLimit("must be 0-100"));
        }
        if lower >= upper {
            return Err(Error::InvalidLimit("lower must be below upper"));
        }

        Ok(Self {
            enabled: true,
            upper,
            lower,
        })
    }

    pub fn apply(&self, device: &HidDevice) -> Result<()> {
        let mut buf = [0u8; 65];
        buf[0] = 0xa0;
        buf[1] = 0x00;
        buf[2] = 0xa0;
        buf[3] = 0x03;
        buf[4] = 0x0b;
        buf[5] = 0x01;
        buf[6] = 0x03;

        if !self.enabled {
            buf[7] = 0x00;
        } else {
            buf[7] = 0x01;
            buf[8] = self.upper;
            buf[9] = self.lower;
        }
        log::error!("{} to {}", self.lower, self.upper);
        log::info!("{:?}", buf);
        device.set_feature(&buf)
    }

    pub fn get_current_from_system(_device: &HidDevice) {
        let _data = [0; 8];
    }
}
