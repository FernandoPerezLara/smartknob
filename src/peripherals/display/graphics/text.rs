use alloc::string::String;

use log::debug;

use super::{Color, Display, Graphic};
use crate::include_generated;

const FONT_BITMAP: &[u8] = include_generated!(bytes: "fonts/primary.bin");

pub struct Text {
    pub content: String,
    pub x: u16,
    pub y: u16,
    pub color: Color,
}

impl Graphic for Text {
    fn draw(&self, display: &mut Display) {
        debug!(
            "Drawing text '{}' at ({}, {}) with color {:?}",
            self.content, self.x, self.y, self.color
        );
    }
}
