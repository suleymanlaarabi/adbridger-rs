use crate::struct_def::{Device, RebootMode};
use crate::utils::get_adb_output;
pub fn reboot(device: &Device, mode: RebootMode) -> Result<String, std::io::Error> {
    let args = [
        "-s",
        device.device_id.as_str(),
        "reboot",
        RebootMode::as_str(&mode),
    ];
    let result = get_adb_output(&args);
    return result;
}
