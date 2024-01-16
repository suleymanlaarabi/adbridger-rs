use core::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Device {
    pub device_id: String,
    pub mode: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BatteryInfo {
    pub device: Device,
    pub battery_level: u8,
}
#[derive(Clone, Copy)]
pub enum RebootMode {
    System,
    Recovery,
}

impl RebootMode {
    pub fn as_str(&self) -> &str {
        match self {
            RebootMode::System => "system",
            RebootMode::Recovery => "recovery",
        }
    }
}

impl Display for RebootMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            RebootMode::Recovery => "recovery",
            RebootMode::System => "system",
        };
        write!(f, "{}", description)
    }
}
