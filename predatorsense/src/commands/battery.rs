use crate::error::Error;
use crate::error::Result;
use crate::hid::HidDevice;

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct ChargingLimit {
    pub enabled: bool,
    pub upper: u8,
    pub lower: u8,
}

/// Used when the device has no limit stored: no upper cap, no lower bound.
const DEFAULT_UPPER: u8 = 100;
const DEFAULT_LOWER: u8 = 0;

impl ChargingLimit {
    pub fn new(enabled: bool, upper: u8, lower: u8) -> Result<Self> {
        validate(upper, lower)?;

        Ok(Self {
            enabled,
            upper,
            lower,
        })
    }

    pub fn apply(&self, device: &HidDevice) -> Result<()> {
        // Last check before the firmware: the struct's fields are public, so
        // a limit can be built without going through `new`.
        if self.enabled {
            validate(self.upper, self.lower)?;
        }

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

        Ok(Self::parse(&buffer))
    }

    /// Reads a charge-limit reply. With the limit disabled the firmware
    /// stores no values and reports 0xff for both, so anything outside 0–100
    /// means "unset" and falls back to the defaults.
    fn parse(buffer: &[u8; 65]) -> Self {
        let in_range = |value: u8, default: u8| if value <= 100 { value } else { default };

        Self {
            enabled: buffer[7] == 1,
            upper: in_range(buffer[8], DEFAULT_UPPER),
            lower: in_range(buffer[9], DEFAULT_LOWER),
        }
    }
}

fn validate(upper: u8, lower: u8) -> Result<()> {
    if upper > 100 || lower > 100 {
        return Err(Error::InvalidLimit("must be 0-100"));
    }
    if lower >= upper {
        return Err(Error::InvalidLimit("lower must be below upper"));
    }
    Ok(())
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

    // parse

    /// A reply with the given bytes 7..=9 and the firmware's 0xff padding after.
    fn reply(enabled: u8, upper: u8, lower: u8) -> [u8; 65] {
        let mut buffer = [0xff; 65];
        buffer[..7].copy_from_slice(&[0xa0, 0x00, 0xe0, 0x03, 0x0b, 0x00, 0x03]);
        buffer[7] = enabled;
        buffer[8] = upper;
        buffer[9] = lower;
        buffer
    }

    #[test]
    fn parse_reads_enabled_limit() {
        assert_eq!(
            ChargingLimit::parse(&reply(1, 80, 60)),
            ChargingLimit {
                enabled: true,
                upper: 80,
                lower: 60
            }
        );
    }

    /// Captured from the device with the limit disabled.
    #[test]
    fn parse_disabled_reply_uses_defaults() {
        assert_eq!(
            ChargingLimit::parse(&reply(0, 0xff, 0xff)),
            ChargingLimit {
                enabled: false,
                upper: DEFAULT_UPPER,
                lower: DEFAULT_LOWER
            }
        );
    }

    #[test]
    fn parse_replaces_each_out_of_range_value_on_its_own() {
        let limit = ChargingLimit::parse(&reply(1, 80, 101));
        assert_eq!(limit.upper, 80);
        assert_eq!(limit.lower, DEFAULT_LOWER);
    }
}
