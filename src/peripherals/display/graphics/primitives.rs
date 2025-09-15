use super::{Color, Figure};
use crate::peripherals::display::Display;
use embedded_graphics::prelude::{Drawable, Point, Primitive};
use embedded_graphics::primitives::{Circle as EgCircle, PrimitiveStyle};
use log::debug;

pub struct FilledCircle {
    pub x: u16,
    pub y: u16,
    pub diameter: u16,
    pub color: Color,
}

impl Figure for FilledCircle {
    fn draw(&self, display: &mut Display) {
        debug!(
            "Drawing filled circle at ({}, {}) with radius {} and color {:?}",
            self.x, self.y, self.diameter, self.color
        );

        let center = Point::new(self.x as i32, self.y as i32);
        let color = self.color.into();

        let circle = EgCircle::with_center(center, self.diameter as u32)
            .into_styled(PrimitiveStyle::with_fill(color));

        let _ = circle.draw(display);
    }
}
