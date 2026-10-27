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
    pub fn new(enabled: bool, upper: u8, lower: u8) -> Result<Self> {
        if upper > 100 || lower > 100 {
            return Err(Error::InvalidLimit("must be 0-100"));
        }
        if lower >= upper {
            return Err(Error::InvalidLimit("lower must be below upper"));
        }

        Ok(Self {
            enabled,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_limits_keeps_values() {
        let limit = ChargingLimit::new(true, 80, 60).unwrap();
        assert_eq!(
            limit,
            ChargingLimit {
                enabled: true,
                upper: 80,
                lower: 60
            }
        );
    }

    #[test]
    fn new_accepts_full_range_edges() {
        assert!(ChargingLimit::new(true, 100, 0).is_ok());
    }

    #[test]
    fn new_rejects_upper_above_100() {
        let result = ChargingLimit::new(true, 101, 50);
        assert!(matches!(result, Err(Error::InvalidLimit(_))));
    }

    #[test]
    fn new_rejects_lower_above_100() {
        let result = ChargingLimit::new(true, 100, 101);
        assert!(matches!(result, Err(Error::InvalidLimit(_))));
    }

    #[test]
    fn new_rejects_lower_equal_to_upper() {
        let result = ChargingLimit::new(true, 60, 60);
        assert!(matches!(result, Err(Error::InvalidLimit(_))));
    }

    #[test]
    fn new_rejects_lower_above_upper() {
        let result = ChargingLimit::new(true, 60, 80);
        assert!(matches!(result, Err(Error::InvalidLimit(_))));
    }
}
