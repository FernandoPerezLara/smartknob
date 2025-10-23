use alloc::{
    format,
    string::{String, ToString},
};

use super::{AppState, Display, View};
use crate::peripherals::display::graphics::{Color, GraphicsError, Text};

pub struct LightView {
    _name: String,
}

impl View for LightView {
    fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
        }
    }

    fn render(&self, state: &AppState, display: &mut Display) -> Result<(), GraphicsError> {
        let text = Text {
            content: format!("Angle: {:.1}", state.angle),
            x: 120,
            y: 140,
            color: Color::WHITE,
        };

        display.draw(&text)?;

        Ok(())
    }
}
