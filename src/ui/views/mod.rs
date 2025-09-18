mod light;

use super::Display;
pub use light::LightView;

pub trait View {
    fn new(name: &str) -> Self
    where
        Self: Sized;
    fn render(&self, display: &mut Display);
}
