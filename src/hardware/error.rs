use core::fmt;
use esp_hal::spi::master::ConfigError as SpiConfigError;

#[derive(Debug)]
pub enum HardwareError {
    Spi(SpiError),
}

#[derive(Debug)]
pub enum SpiError {
    Config(SpiConfigError),
    InvalidParameters(&'static str),
    TransferFailed(&'static str),
    WriteFailed(&'static str),
    ReadFailed(&'static str),
}

impl HardwareError {}

impl SpiError {
    pub fn invalid_parameters(msg: &'static str) -> Self {
        Self::InvalidParameters(msg)
    }

    pub fn transfer_failed(msg: &'static str) -> Self {
        Self::TransferFailed(msg)
    }

    pub fn write_failed(msg: &'static str) -> Self {
        Self::WriteFailed(msg)
    }

    pub fn read_failed(msg: &'static str) -> Self {
        Self::ReadFailed(msg)
    }
}

impl From<SpiError> for HardwareError {
    fn from(err: SpiError) -> Self {
        Self::Spi(err)
    }
}

impl From<SpiConfigError> for SpiError {
    fn from(err: SpiConfigError) -> Self {
        Self::Config(err)
    }
}

impl From<SpiConfigError> for HardwareError {
    fn from(err: SpiConfigError) -> Self {
        Self::Spi(SpiError::Config(err))
    }
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spi(err) => write!(f, "SPI error: {}", err),
        }
    }
}

impl fmt::Display for SpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(err) => write!(f, "SPI configuration error: {:?}", err),
            Self::InvalidParameters(msg) => write!(f, "Invalid SPI parameters: {}", msg),
            Self::TransferFailed(msg) => write!(f, "SPI transfer failed: {}", msg),
            Self::WriteFailed(msg) => write!(f, "SPI write failed: {}", msg),
            Self::ReadFailed(msg) => write!(f, "SPI read failed: {}", msg),
        }
    }
}
