use core::fmt;

use crate::hardware::error::SpiError;

#[derive(Debug)]
pub enum EncoderError {
    Spi(SpiError),
}

impl From<SpiError> for EncoderError {
    fn from(err: SpiError) -> Self {
        Self::Spi(err)
    }
}

impl fmt::Display for EncoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spi(err) => write!(f, "SPI error in encoder: {}", err),
        }
    }
}
