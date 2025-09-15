mod color;
mod primitives;

pub use self::color::Color;
pub use self::primitives::FilledCircle;
use crate::peripherals::display::error::DisplayError;
use crate::peripherals::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{
    Dimensions, DrawTarget, IntoStorage, OriginDimensions, Pixel, Size,
};

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}

impl DrawTarget for Display {
    type Color = Rgb565;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|&Pixel(pos, _color)| bb.contains(pos))
            .for_each(|Pixel(position, color)| {
                let raw_color = color.into_storage();

                self.set_pixel(position.x as u16, position.y as u16, raw_color);
            });

        Ok(())
    }
}

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
