use crate::struct_def::Device;
use crate::utils::get_adb_output;

pub fn list_devices() -> Result<Vec<Device>, std::io::Error> {
    let args = ["devices"];
    match get_adb_output(&args) {
        Ok(output) => {
            let devices: Vec<Device> = output
                .split("\n")
                .filter(|&s| s != "" && s != "List of devices attached")
                .map(|line| {
                    let parts: Vec<&str> = line.split('\t').collect();
                    Device {
                        device_id: parts[0].to_string(),
                        mode: parts.get(1).unwrap_or(&"unknown").to_string(),
                    }
                })
                .collect();
            Ok(devices)
        }
        Err(e) => Err(e),
    }
}
