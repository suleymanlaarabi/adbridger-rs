use adbridger::{device, screen::display_screen};

fn main() {
    // device::wait_for_device().unwrap();
    let devices = device::list_devices().unwrap();
    println!("{:?}", devices[0].device_id);
    // let android_version = device::android::get_version(&devices[0]);
    display_screen(&devices[0]).unwrap();
    //   println!("{:?}", android_version);
}
