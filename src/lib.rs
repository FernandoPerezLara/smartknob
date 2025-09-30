#![no_std]

extern crate alloc;

pub mod app;
pub mod error;
pub mod hardware;
pub mod macros;
pub mod peripherals;
pub mod ui;

pub use app::App;
