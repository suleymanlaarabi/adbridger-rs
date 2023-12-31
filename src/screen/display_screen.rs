use crate::{struct_def::Device, utils::start_scrcpy};

pub fn display_screen(device: &Device) -> Result<String, std::io::Error> {
    start_scrcpy(&["-b", "4M", "-s", device.device_id.as_str()])
}
