use alloc::string::String;

use log::debug;

use super::{Color, Display, Graphic, GraphicsError};
use crate::include_generated;

const FONT_BITMAP: &[u8] = include_generated!(bytes: "fonts/primary.bin");

struct FontHeader {
    char_count: u32,
    baseline: u32,
}

struct BinaryFont<'a> {
    header: FontHeader,
    char_map: &'a [u8],
    glyph_metadata: &'a [u8],
    bitmap_data: &'a [u8],
}

impl<'a> BinaryFont<'a> {
    fn new(data: &'a [u8]) -> Result<Self, GraphicsError> {
        if data.len() < 8 {
            return Err(GraphicsError::InvalidFontData(
                "Font data too short for header",
            ));
        }

        let char_count = u32::from_le_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| GraphicsError::InvalidFontData("Invalid char_count bytes"))?,
        );
        let baseline = u32::from_le_bytes(
            data[4..8]
                .try_into()
                .map_err(|_| GraphicsError::InvalidFontData("Invalid baseline bytes"))?,
        );

        let char_map_start = 8;
        let char_map_size = (char_count as usize) * 2;
        let char_map_end = char_map_start + char_map_size;

        if data.len() < char_map_end {
            return Err(GraphicsError::InvalidFontData(
                "Font data too short for char map",
            ));
        }

        let glyph_metadata_start = char_map_end;
        let glyph_metadata_size = (char_count as usize) * 6;
        let glyph_metadata_end = glyph_metadata_start + glyph_metadata_size;

        if data.len() < glyph_metadata_end {
            return Err(GraphicsError::InvalidFontData(
                "Font data too short for glyph metadata",
            ));
        }

        let header = FontHeader {
            char_count,
            baseline,
        };
        let char_map = &data[char_map_start..char_map_end];
        let glyph_metadata = &data[glyph_metadata_start..glyph_metadata_end];
        let bitmap_data = &data[glyph_metadata_end..];

        Ok(Self {
            header,
            char_map,
            glyph_metadata,
            bitmap_data,
        })
    }
}

pub struct Text {
    pub content: String,
    pub x: u16,
    pub y: u16,
    pub color: Color,
}

impl Graphic for Text {
    fn draw(&self, _display: &mut Display) -> Result<(), GraphicsError> {
        debug!(
            "Drawing text '{}' at ({}, {}) with color {:?}",
            self.content, self.x, self.y, self.color
        );

        let font = BinaryFont::new(FONT_BITMAP)?;
        let color = self.color.into();

        for ch in self.content.chars() {

        }

        Ok(())
    }
}
