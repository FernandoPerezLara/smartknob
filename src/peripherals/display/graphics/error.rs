use core::fmt;

use super::DisplayError;

#[derive(Debug)]
pub enum GraphicsError {
    InvalidFontData(&'static str),
    FontCharacterNotFound(char),
    InvalidCoordinates { x: u16, y: u16 },
    DisplayError,
}

impl From<DisplayError> for GraphicsError {
    fn from(_err: DisplayError) -> Self {
        Self::DisplayError
    }
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFontData(msg) => write!(f, "Invalid font data: {}", msg),
            Self::FontCharacterNotFound(ch) => {
                write!(f, "Character '{}' not found in font", ch)
            },
            Self::InvalidCoordinates { x, y } => {
                write!(f, "Invalid coordinates: ({}, {})", x, y)
            },
            Self::DisplayError => write!(f, "Display error occurred"),
        }
    }
}
