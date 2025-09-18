use core::fmt;

use crate::{hardware::error::HardwareError, peripherals::display::error::DisplayError};

#[derive(Debug)]
pub enum SmartknobError {
    Hardware(HardwareError),
    Display(DisplayError),
}

impl From<HardwareError> for SmartknobError {
    fn from(err: HardwareError) -> Self {
        Self::Hardware(err)
    }
}

impl From<DisplayError> for SmartknobError {
    fn from(err: DisplayError) -> Self {
        Self::Display(err)
    }
}

impl fmt::Display for SmartknobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hardware(err) => write!(f, "Hardware error: {}", err),
            Self::Display(err) => write!(f, "Display error: {}", err),
        }
    }
}
