use std::fs::File;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

static mut SCRCPY_PATH: Option<PathBuf> = None;

pub fn initialize_scrcpy() -> io::Result<()> {
    let temp_dir = std::env::temp_dir();
    let scrcpy_path = temp_dir.join("scrcpy");
    let mut file = File::create(&scrcpy_path)?;
    file.write_all(include_bytes!("../../assets/bin/linux/scrcpy"))?;
    file.set_permissions(std::fs::Permissions::from_mode(0o755))?;

    unsafe {
        SCRCPY_PATH = Some(scrcpy_path);
    }

    Ok(())
}

pub fn start_scrcpy(args: &[&str]) -> Result<String, io::Error> {
    let scrcpy_path = unsafe { SCRCPY_PATH.as_ref().expect("scrcpy n'a pas été initialisé") };

    let result = Command::new(scrcpy_path).args(args).output()?;
    let data = String::from_utf8(result.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if result.status.success() {
        Ok(data)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, data))
    }
}
