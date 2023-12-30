use adbridger::{device, screen::display_screen};
fn main() {
    device::wait_for_device().unwrap();
    let devices = device::list_devices().unwrap();
    display_screen(&devices[0]).unwrap();

    let android_version = device::android::get_version(&devices[0]).unwrap();
    println!("{:?}", android_version);
}
