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

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled
    }

    pub fn set_upper(&mut self, upper: u8) -> Result<()> {
        if upper > 100 {
            return Err(Error::InvalidLimit(
                "upper limit cannot be greater than 100",
            ));
        }
        if upper < self.lower {
            return Err(Error::InvalidLimit("upper must be above lower"));
        }
        self.upper = upper;

        Ok(())
    }

    pub fn set_lwoer(&mut self, lower: u8) -> Result<()> {
        if lower > 100 {
            return Err(Error::InvalidLimit(
                "lower limit cannot be greater than 100",
            ));
        }
        if lower > self.upper {
            return Err(Error::InvalidLimit("lower must be below upper"));
        }
        self.lower = lower;

        Ok(())
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
        device.set_feature(&buf)
    }

    pub fn from_system(device: &HidDevice) -> Result<Self> {
        const REPORT_ID: u8 = 0xa0;
        let request = [REPORT_ID, 0x00, 0xa0, 0x03, 0x0b, 0x02, 0x00, 0x00];
        device.set_feature(&request)?;

        let mut buffer = [0u8; 65];
        buffer[0] = REPORT_ID;

        device.get_feature_report(&mut buffer)?;

        let enabled = buffer[7] == 1;
        let upper = buffer[8];
        let lower = buffer[9];

        Ok(Self {
            enabled,
            upper,
            lower,
        })
    }
}
