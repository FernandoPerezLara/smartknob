use core::fmt;

use crate::{
    hardware::error::HardwareError,
    peripherals::{
        display::{error::DisplayError, graphics::GraphicsError},
        encoder::error::EncoderError,
    },
};

#[derive(Debug)]
pub enum SmartknobError {
    Hardware(HardwareError),
    Display(DisplayError),
    Graphics(GraphicsError),
    Encoder(EncoderError),
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

impl From<GraphicsError> for SmartknobError {
    fn from(err: GraphicsError) -> Self {
        Self::Graphics(err)
    }
}

impl From<EncoderError> for SmartknobError {
    fn from(err: EncoderError) -> Self {
        Self::Encoder(err)
    }
}

impl fmt::Display for SmartknobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hardware(err) => write!(f, "Hardware error: {}", err),
            Self::Display(err) => write!(f, "Display error: {}", err),
            Self::Graphics(err) => write!(f, "Graphics error: {}", err),
            Self::Encoder(err) => write!(f, "Encoder error: {}", err),
        }
    }
}
