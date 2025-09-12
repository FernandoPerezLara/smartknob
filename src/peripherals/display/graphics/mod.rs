mod primitives;

pub use self::primitives::*;
use crate::peripherals::display::Display;

pub trait Figure {
    fn draw(&self, display: &mut Display);
}

impl Display {
    pub fn draw<T>(&mut self, shape: &T)
    where
        T: Figure,
    {
        shape.draw(self);
    }
}
