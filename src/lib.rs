#![no_std]

pub mod app;
pub mod error;
pub mod hardware;
pub mod peripherals;

pub use error::{ApplicationError, DisplayError, HardwareError, SmartknobError};
pub use hardware::Hardware;

pub type Result<T> = core::result::Result<T, SmartknobError>;

pub type HardwareResult<T> = core::result::Result<T, HardwareError>;

pub type DisplayResult<T> = core::result::Result<T, DisplayError>;

pub type AppResult<T> = core::result::Result<T, ApplicationError>;
