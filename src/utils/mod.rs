mod adb_output;
pub use adb_output::get_adb_output;

mod scrcpy_output;
pub use scrcpy_output::start_scrcpy;

mod check_dependencies;
pub use check_dependencies::check_and_install_dependencies;
