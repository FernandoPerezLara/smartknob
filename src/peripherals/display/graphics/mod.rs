mod primitives;

pub use self::primitives::*;
use crate::peripherals::display::{Display, DisplayError};
use core::future::Future;

pub trait Figure {
    fn draw(&self, display: &mut Display) -> impl Future<Output = Result<(), DisplayError>>;
}
