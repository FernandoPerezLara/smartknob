use crate::hardware::error::SpiError;
use core::fmt;

#[derive(Debug)]
pub enum DisplayError {
    Spi(SpiError),
    InvalidOperation(&'static str),
}

impl DisplayError {
    pub fn invalid_operation(msg: &'static str) -> Self {
        Self::InvalidOperation(msg)
    }
}

impl From<SpiError> for DisplayError {
    fn from(err: SpiError) -> Self {
        Self::Spi(err)
    }
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spi(err) => write!(f, "SPI error in display: {}", err),
            Self::InvalidOperation(msg) => write!(f, "Invalid display operation: {}", msg),
        }
    }
}
