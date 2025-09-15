use super::Figure;
use crate::peripherals::display::Display;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Drawable, Point, Primitive};
use embedded_graphics::primitives::{Circle as EgCircle, Line as EgLine, PrimitiveStyle};
use log::debug;

pub struct Line {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub color: u16,
}

impl Figure for Line {
    fn draw(&self, display: &mut Display) {
        debug!(
            "Drawing line from ({}, {}) to ({}, {}) with color 0x{:04X}",
            self.x1, self.y1, self.x2, self.y2, self.color
        );

        let start = Point::new(self.x1 as i32, self.y1 as i32);
        let end = Point::new(self.x2 as i32, self.y2 as i32);
        let color = Rgb565::new(
            ((self.color >> 11) & 0x1F) as u8,
            ((self.color >> 5) & 0x3F) as u8,
            (self.color & 0x1F) as u8,
        );

        let line = EgLine::new(start, end).into_styled(PrimitiveStyle::with_stroke(color, 1));

        let _ = line.draw(display);
    }
}

pub struct Circle {
    pub x: u16,
    pub y: u16,
    pub diameter: u16,
    pub color: u16,
}

impl Figure for Circle {
    fn draw(&self, display: &mut Display) {
        debug!(
            "Drawing circle at ({}, {}) with radius {} and color 0x{:04X}",
            self.x, self.y, self.diameter, self.color
        );

        let center = Point::new(self.x as i32, self.y as i32);
        let color = Rgb565::new(
            ((self.color >> 11) & 0x1F) as u8,
            ((self.color >> 5) & 0x3F) as u8,
            (self.color & 0x1F) as u8,
        );

        let circle = EgCircle::with_center(center, self.diameter as u32)
            .into_styled(PrimitiveStyle::with_stroke(color, 1));

        let _ = circle.draw(display);
    }
}

pub struct FilledCircle {
    pub x: u16,
    pub y: u16,
    pub diameter: u16,
    pub color: u16,
}

impl Figure for FilledCircle {
    fn draw(&self, display: &mut Display) {
        debug!(
            "Drawing filled circle at ({}, {}) with radius {} and color 0x{:04X}",
            self.x, self.y, self.diameter, self.color
        );

        let center = Point::new(self.x as i32, self.y as i32);
        let color = Rgb565::new(
            ((self.color >> 11) & 0x1F) as u8,
            ((self.color >> 5) & 0x3F) as u8,
            (self.color & 0x1F) as u8,
        );

        let circle = EgCircle::with_center(center, self.diameter as u32)
            .into_styled(PrimitiveStyle::with_fill(color));

        let _ = circle.draw(display);
    }
}
