use crate::utils::get_adb_output;
pub fn wait_for_device() -> Result<String, std::io::Error> {
    let args = ["wait-for-device"];
    let result = get_adb_output(&args);
    return result;
}
