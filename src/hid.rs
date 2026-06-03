use hidapi::{HidApi, HidDevice as RawHidDevice};
use crate::error::{Error, Result};

pub struct HidDevice {
    inner: RawHidDevice,
    vid: u16, // Vendor ID
    pid: u16 // Product ID
}

impl HidDevice {
    pub fn open_by_vid_pid(api: &HidApi, vid: u16, pid: u16) -> Result<Self>{
        let inner = api.open(vid, pid).map_err(|e| {
            let msg = format!("{e}");
              if msg.contains("Permission") || msg.contains("permission") {
                  Error::PermissionDenied
              } else {
                  Error::NotFound { vid, pid }
              }
        })?;

        Ok(Self{inner, vid, pid})
    }

    pub fn set_feature(&self, buf: &[u8]) -> Result<()> {
        self.inner.send_feature_report(buf)?;

        Ok(())
    }

}