use crate::struct_def::{BatteryInfo, Device};
use crate::utils::get_adb_output;

pub fn get_battery_info(device: &Device) -> Result<BatteryInfo, std::io::Error> {
    let args: [&str; 5] = [
        "-s",
        device.device_id.as_str(),
        "shell",
        "dumpsys",
        "battery",
    ];
    match get_adb_output(&args) {
        Ok(output) => {
            let index_of_battery_level = output.find("level:").expect("Battery not available");
            let device_info_string: u8 = output
                [index_of_battery_level + 6..index_of_battery_level + 11]
                .trim()
                .parse()
                .expect("Error for get battery level");
            Result::Ok(BatteryInfo {
                device: Device {
                    device_id: device.device_id.clone(),
                    mode: device.mode.clone(),
                },
                battery_level: device_info_string,
            })
        }
        Err(e) => Err(e),
    }
}
