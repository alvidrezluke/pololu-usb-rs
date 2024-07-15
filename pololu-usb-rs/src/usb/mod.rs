use nusb::DeviceInfo;

pub fn get_devices() -> Vec<DeviceInfo> {
    nusb::list_devices()
        .expect("Was unable to access any devices")
        .collect()
}
