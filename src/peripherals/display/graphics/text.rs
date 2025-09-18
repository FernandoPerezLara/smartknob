use alloc::string::String;

pub use embedded_graphics::text::Alignment;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    prelude::{Drawable as EgDrawable, Point},
    text::{Baseline, LineHeight, Text as EgText, TextStyleBuilder},
};
use log::debug;

use super::{Color, Graphic};
use crate::peripherals::display::Display;

pub struct Text {
    pub content: String,
    pub x: u16,
    pub y: u16,
    pub alignment: Alignment,
    pub color: Color,
}

impl Graphic for Text {
    fn draw(&self, display: &mut Display) {
        debug!(
            "Drawing text '{}' at ({}, {}) with color {:?}",
            self.content, self.x, self.y, self.color
        );

        let position = Point::new(self.x as i32, self.y as i32);
        let color = self.color.into();
        let character_style = MonoTextStyle::new(&FONT_10X20, color);
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .line_height(LineHeight::Percent(150))
            .build();

        let text = EgText::with_text_style(&self.content, position, character_style, text_style);

        let _ = text.draw(display);
    }
}
