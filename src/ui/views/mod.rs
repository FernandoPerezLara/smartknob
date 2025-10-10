mod light;

pub use light::LightView;

use super::{Display, GraphicsError};

pub trait View {
    fn new(name: &str) -> Self
    where
        Self: Sized;
    fn render(&self, display: &mut Display) -> Result<(), GraphicsError>;
}
