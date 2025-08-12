use core::fmt;
use esp_hal::spi::master::ConfigError as SpiConfigError;

#[derive(Debug)]
pub enum SmartknobError {
    Hardware(HardwareError),
    Display(DisplayError),
    Application(ApplicationError),
}

#[derive(Debug)]
pub enum HardwareError {
    SpiConfig(SpiConfigError),
    SpiWrite,
    SpiRead,
    SpiTransfer,
    GpioConfig,
    InitializationFailed,
}

#[derive(Debug)]
pub enum DisplayError {
    InitializationFailed,
    CommunicationFailed,
    InvalidCommand,
    DriverError,
}

#[derive(Debug)]
pub enum ApplicationError {
    InvalidStateTransition,
    ConfigurationError,
    ResourceUnavailable,
}

impl From<SpiConfigError> for HardwareError {
    fn from(err: SpiConfigError) -> Self {
        HardwareError::SpiConfig(err)
    }
}

impl From<HardwareError> for SmartknobError {
    fn from(err: HardwareError) -> Self {
        SmartknobError::Hardware(err)
    }
}

impl From<DisplayError> for SmartknobError {
    fn from(err: DisplayError) -> Self {
        SmartknobError::Display(err)
    }
}

impl From<ApplicationError> for SmartknobError {
    fn from(err: ApplicationError) -> Self {
        SmartknobError::Application(err)
    }
}

impl From<SpiConfigError> for SmartknobError {
    fn from(err: SpiConfigError) -> Self {
        SmartknobError::Hardware(HardwareError::SpiConfig(err))
    }
}

impl fmt::Display for SmartknobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmartknobError::Hardware(e) => write!(f, "Hardware error: {}", e),
            SmartknobError::Display(e) => write!(f, "Display error: {}", e),
            SmartknobError::Application(e) => write!(f, "Application error: {}", e),
        }
    }
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HardwareError::SpiConfig(e) => write!(f, "SPI configuration failed: {:?}", e),
            HardwareError::SpiWrite => write!(f, "SPI transfer failed"),
            HardwareError::SpiRead => write!(f, "SPI read failed"),
            HardwareError::SpiTransfer => write!(f, "SPI transfer failed"),
            HardwareError::GpioConfig => write!(f, "GPIO configuration failed"),
            HardwareError::InitializationFailed => write!(f, "Hardware initialization failed"),
        }
    }
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayError::InitializationFailed => write!(f, "Display initialization failed"),
            DisplayError::CommunicationFailed => write!(f, "Display communication failed"),
            DisplayError::InvalidCommand => write!(f, "Invalid display command"),
            DisplayError::DriverError => write!(f, "Display driver error"),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::InvalidStateTransition => write!(f, "Invalid state transition"),
            ApplicationError::ConfigurationError => write!(f, "Configuration error"),
            ApplicationError::ResourceUnavailable => write!(f, "Resource unavailable"),
        }
    }
}
