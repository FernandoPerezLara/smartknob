use core::fmt;

use crate::hardware::error::SpiError;

#[derive(Debug)]
pub enum DisplayError {
    Spi(SpiError),
    InvalidOperation(&'static str),
    OutOfBounds { x1: u16, y1: u16, x2: u16, y2: u16 },
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
            Self::OutOfBounds { x1, y1, x2, y2 } => write!(
                f,
                "Coordinates out of bounds: ({}, {}) >= ({}, {})",
                x1, y1, x2, y2
            ),
        }
    }
}
