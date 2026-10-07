use crate::error;
use crate::hid::HidDevice;
pub struct Config {
    rgb: Option<HidDevice>,
    system: Option<HidDevice>,
}

impl Default for Config {
    fn default() -> Self {
        Self::open()
    }
}
impl Config {
    pub fn open() -> Self {
        let led_vid: u16 = 3314;
        let led_pid: u16 = 20784;

        let system_vid: u16 = 4133;
        let system_pid: u16 = 5963;

        let rgb = HidDevice::open_by_vid_pid(led_vid, led_pid).ok();
        let system = HidDevice::open_by_vid_pid(system_vid, system_pid).ok();

        let new_rgb = rgb.unwrap();
        Self {
            rgb: Some(new_rgb),
            system,
        }
    }
    pub fn offline() -> Self {
        Self {
            rgb: None,
            system: None,
        }
    }
    pub fn rgb(&self) -> Result<&HidDevice, error::Error> {
        self.rgb.as_ref().ok_or(error::Error::Unavailable {
            role: "rgb",
            reason: "Could not find RGB device".to_string(),
        })
    }
    pub fn system(&self) -> Result<&HidDevice, error::Error> {
        self.system.as_ref().ok_or(error::Error::Unavailable {
            role: "rgb",
            reason: "Could not find RGB device".to_string(),
        })
    }
}
