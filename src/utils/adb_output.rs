use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::str;

pub fn get_adb_output(args: &[&str]) -> Result<String, std::io::Error> {
    let adb_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/bin/linux/adb");

    let result = Command::new(adb_path).args(args).output()?;
    let data =
        String::from_utf8(result.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
    if result.status.success() {
        return data;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, data.expect("_")));
    }
}
