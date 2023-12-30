use std::fs::File;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
static mut ADB_PATH: Option<PathBuf> = None;

pub fn initialize_adb() -> io::Result<()> {
    let temp_dir = std::env::temp_dir();
    let adb_path = temp_dir.join("adb");
    let mut file = File::create(&adb_path)?;
    file.write_all(include_bytes!("../../assets/bin/linux/adb"))?;
    file.set_permissions(std::fs::Permissions::from_mode(0o755))?;

    unsafe {
        ADB_PATH = Some(adb_path);
    }

    Ok(())
}

pub fn get_adb_output(args: &[&str]) -> Result<String, io::Error> {
    let adb_path = unsafe { ADB_PATH.as_ref().expect("ADB is not started") };

    let result = Command::new(adb_path).args(args).output()?;
    let data = String::from_utf8(result.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if result.status.success() {
        Ok(data)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, data))
    }
}
