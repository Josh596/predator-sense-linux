use crate::hid::HidDevice;

struct Config {
    led_hid_device: HidDevice,
    battery_hid_device: HidDevice,
}
