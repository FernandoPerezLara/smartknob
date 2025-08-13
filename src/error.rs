pub use crate::hardware::HardwareError;
use core::fmt;

#[derive(Debug)]
pub enum SmartknobError {
    Hardware(HardwareError),
}

impl From<HardwareError> for SmartknobError {
    fn from(err: HardwareError) -> Self {
        Self::Hardware(err)
    }
}

impl fmt::Display for SmartknobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hardware(err) => write!(f, "Hardware error: {}", err),
        }
    }
}
