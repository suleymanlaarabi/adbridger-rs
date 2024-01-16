use std::process::Command;


pub fn check_and_install_dependencies() -> Result<(), std::io::Error> {
    // Liste des dépendances à installer
    let dependencies = [
        "ffmpeg", "libsdl2-2.0-0", "adb", "wget", "gcc", "git", "pkg-config", 
        "meson", "ninja-build", "libsdl2-dev", "libavcodec-dev", "libavdevice-dev",
        "libavformat-dev", "libavutil-dev", "libswresample-dev", "libusb-1.0-0",
        "libusb-1.0-0-dev"
    ];

    // Construire la commande
    let mut command = Command::new("sudo");
    command.arg("apt").arg("install");
    for dep in &dependencies {
        command.arg(dep);
    }

    // Exécuter la commande
    let status = command.status()?;

    if !status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "La commande a échoué"));
    }

    Ok(())
}