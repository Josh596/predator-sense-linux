use crate::error::{Error, Result};
use hidapi::{HidApi, HidDevice as RawHidDevice};

#[derive(Debug)]
pub struct HidDevice {
    inner: RawHidDevice,
    vid: u16, // Vendor ID
    pid: u16, // Product ID
}

impl HidDevice {
    pub fn open_by_vid_pid(vid: u16, pid: u16) -> Result<Self> {
        let api = match HidApi::new() {
            Ok(api) => api,
            Err(e) => return Err(Error::Hid(e)),
        };

        let inner = api.open(vid, pid).map_err(|e| {
            let msg = format!("{e}");
            if msg.contains("Permission") || msg.contains("permission") {
                Error::PermissionDenied
            } else {
                Error::NotFound { vid, pid }
            }
        })?;

        Ok(Self { inner, vid, pid })
    }

    pub fn set_feature(&self, buf: &[u8]) -> Result<()> {
        self.inner.send_feature_report(buf)?;

        Ok(())
    }

    pub fn get_feature_report(&self, mut buf: &mut [u8]) -> Result<usize> {
        self.inner.get_feature_report(&mut buf).map_err(Error::Hid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Talks to the real hidapi, but needs no hardware: no device has id 0000:0000.
    #[test]
    fn open_missing_device_is_not_found() {
        let err = HidDevice::open_by_vid_pid(0x0000, 0x0000).unwrap_err();
        assert!(
            matches!(err, Error::NotFound { vid: 0, pid: 0 }),
            "got {err:?}"
        );
    }
}
