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
            log::error!("{msg}");
            if msg.contains("Permission") || msg.contains("permission") {
                Error::PermissionDenied
            } else {
                Error::NotFound { vid, pid }
            }
        })?;

        Ok(Self { inner, vid, pid })
    }

    pub fn set_feature(&self, buf: &[u8]) -> Result<()> {
        log::info!("Set feature called");
        log::info!("{:?} -> {:?}", self, self.inner.get_device_info().unwrap());
        self.inner.send_feature_report(buf).unwrap();

        Ok(())
    }
}
