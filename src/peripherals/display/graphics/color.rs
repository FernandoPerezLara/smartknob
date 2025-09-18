use core::fmt;

use embedded_graphics::pixelcolor::Rgb565;

#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Color {
    pub const fn to_rgb565(&self) -> u16 {
        let r = (self.0 >> 3) as u16;
        let g = (self.1 >> 2) as u16;
        let b = (self.2 >> 3) as u16;

        (r << 11) | (g << 5) | b
    }

    pub fn to_embedded_rgb565(&self) -> Rgb565 {
        Rgb565::new(self.0 >> 3, self.1 >> 2, self.2 >> 3)
    }

    pub const BLACK: Color = Color(0, 0, 0);
    pub const WHITE: Color = Color(255, 255, 255);
}

impl From<Color> for u16 {
    fn from(color: Color) -> Self {
        color.to_rgb565()
    }
}

impl From<Color> for Rgb565 {
    fn from(color: Color) -> Self {
        color.to_embedded_rgb565()
    }
}
