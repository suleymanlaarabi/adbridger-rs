use crate::struct_def::Device;
use crate::utils::get_adb_output;
pub fn pull(
    input_path: &str,
    output_path: &str,
    device: &Device,
) -> Result<String, std::io::Error> {
    let args = [
        "-s",
        device.device_id.as_str(),
        "pull",
        input_path,
        output_path,
    ];
    let result = get_adb_output(&args);
    return result;
}
