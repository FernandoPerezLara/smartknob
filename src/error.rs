use core::fmt;

use esp_hal::spi::master::ConfigError as SpiConfigError;

#[derive(Debug)]
pub enum SmartknobError {
    Hardware(&'static str),
    Spi(&'static str),
    Initialization(&'static str),
    Application(&'static str),
    SpiConfig(SpiConfigError),
}

impl SmartknobError {
    pub fn hardware(msg: &'static str) -> Self {
        Self::Hardware(msg)
    }

    pub fn spi(msg: &'static str) -> Self {
        Self::Spi(msg)
    }

    pub fn initialization(msg: &'static str) -> Self {
        Self::Initialization(msg)
    }

    pub fn application(msg: &'static str) -> Self {
        Self::Application(msg)
    }
}

impl From<SpiConfigError> for SmartknobError {
    fn from(err: SpiConfigError) -> Self {
        Self::SpiConfig(err)
    }
}

impl fmt::Display for SmartknobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hardware(msg) => write!(f, "Hardware error: {}", msg),
            Self::Spi(msg) => write!(f, "SPI error: {}", msg),
            Self::Initialization(msg) => write!(f, "Initialization error: {}", msg),
            Self::Application(msg) => write!(f, "Application error: {}", msg),
            Self::SpiConfig(err) => write!(f, "SPI configuration error: {:?}", err),
        }
    }
}
