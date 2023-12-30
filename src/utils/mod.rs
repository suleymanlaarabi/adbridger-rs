mod adb_output;
pub use adb_output::get_adb_output;
pub use adb_output::initialize_adb;

mod scrcpy_output;
pub use scrcpy_output::initialize_scrcpy;
pub use scrcpy_output::start_scrcpy;
