use crate::error::Result;
use crate::hid::HidDevice;

fn set_backlight_timeout(device: &HidDevice, seconds: u8) -> Result<()> {
    let mut buf = [0u8; 65];

    buf[0] = 0xa0;
    buf[1] = 0x0;
    buf[2] = 0xa0;
    buf[3] = 0x0a;
    buf[4] = 0x0;
    buf[5] = 0x01;
    buf[6] = 0x02;
    buf[7] = 0x01;
    buf[8] = 0x0;
    buf[9] = 0x64;
    buf[10] = 0x00;
    buf[11] = seconds;

    device.set_feature(&buf)
}
