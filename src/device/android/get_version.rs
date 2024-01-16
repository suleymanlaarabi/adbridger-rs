use crate::struct_def::Device;
use crate::utils::get_adb_output;

pub fn get_version(device: &Device) -> Result<u8, std::io::Error> {
    let args = [
        "-s",
        device.device_id.as_str(),
        "shell",
        "getprop",
        "ro.build.version.release",
    ];
    let result: Result<String, std::io::Error> = get_adb_output(&args);
    match result {
        Ok(e) => Ok(e.trim().parse().expect("Unable to get android version")),
        Err(e) => Err(e),
    }
}
