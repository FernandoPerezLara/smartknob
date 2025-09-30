use alloc::string::{String, ToString};

use super::{Display, View};
use crate::peripherals::display::graphics::{Color, Text};

pub struct LightView {
    name: String,
}

impl View for LightView {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn render(&self, display: &mut Display) {
        let text = Text {
            content: self.name.clone(),
            x: 120,
            y: 120,
            color: Color::WHITE,
        };

        display.draw(&text);
    }
}
