use alloc::string::String;
use core::cmp::Ordering;

use log::debug;

use super::{Color, Display, Graphic, GraphicsError};
use crate::include_generated;

const FONT_BITMAP: &[u8] = include_generated!(bytes: "fonts/primary.bin");

struct BinaryFont<'a> {
    char_count: u32,
    char_map: &'a [u8],
    glyph_metadata: &'a [u8],
    bitmap_data: &'a [u8],
}

struct GlyphMetadata {
    width: u8,
    height: u8,
    xmin: i8,
    ymin: i8,
    advance_width: u8,
    bitmap_offset: u16,
}

fn blend_rgb565(foreground: u16, background: u16, alpha: u8) -> u16 {
    let background_color = (
        (background >> 11) & 0x1F,
        (background >> 5) & 0x3F,
        background & 0x1F,
    );
    let foreground_color = (
        (foreground >> 11) & 0x1F,
        (foreground >> 5) & 0x3F,
        foreground & 0x1F,
    );

    let alpha_u16 = alpha as u16;
    let inv_alpha = 15 - alpha_u16;

    (((foreground_color.0 * alpha_u16 + background_color.0 * inv_alpha) / 15) & 0x1F) << 11
        | (((foreground_color.1 * alpha_u16 + background_color.1 * inv_alpha) / 15) & 0x3F) << 5
        | (((foreground_color.2 * alpha_u16 + background_color.2 * inv_alpha) / 15) & 0x1F)
}

impl<'a> BinaryFont<'a> {
    fn new(data: &'a [u8]) -> Result<Self, GraphicsError> {
        if data.len() < 4 {
            return Err(GraphicsError::InvalidFontData(
                "Font data too short for header",
            ));
        }

        let char_count = u32::from_le_bytes(
            data[0..4]
                .try_into()
                .map_err(|_| GraphicsError::InvalidFontData("Invalid char_count bytes"))?,
        );

        let char_map_start = 4;
        let char_map_size = (char_count as usize) * 2;
        let char_map_end = char_map_start + char_map_size;

        if data.len() < char_map_end {
            return Err(GraphicsError::InvalidFontData(
                "Font data too short for char map",
            ));
        }

        let glyph_metadata_start = char_map_end;
        let glyph_metadata_size = (char_count as usize) * 7;
        let glyph_metadata_end = glyph_metadata_start + glyph_metadata_size;

        if data.len() < glyph_metadata_end {
            return Err(GraphicsError::InvalidFontData(
                "Font data too short for glyph metadata",
            ));
        }

        Ok(Self {
            char_count,
            char_map: &data[char_map_start..char_map_end],
            glyph_metadata: &data[glyph_metadata_start..glyph_metadata_end],
            bitmap_data: &data[glyph_metadata_end..],
        })
    }

    fn find_glyph(&self, ch: char) -> Result<GlyphMetadata, GraphicsError> {
        let target_char = ch as u16;

        let mut low = 0;
        let mut high = self.char_count as usize;

        while low < high {
            let mid = low + (high - low) / 2;
            let mid_char = u16::from_le_bytes([self.char_map[mid * 2], self.char_map[mid * 2 + 1]]);

            match mid_char.cmp(&target_char) {
                Ordering::Equal => {
                    let offset = mid * 7;
                    let metadata = &self.glyph_metadata[offset..offset + 7];

                    return Ok(GlyphMetadata {
                        width: metadata[0],
                        height: metadata[1],
                        xmin: metadata[2] as i8,
                        ymin: metadata[3] as i8,
                        advance_width: metadata[4],
                        bitmap_offset: u16::from_le_bytes([metadata[5], metadata[6]]),
                    });
                },
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }

        Err(GraphicsError::FontCharacterNotFound(ch))
    }

    fn render_glyph(
        &self,
        glyph: &GlyphMetadata,
        x: i32,
        y: i32,
        color: u16,
        display: &mut Display,
    ) {
        let x_offset = x + glyph.xmin as i32;
        let y_offset = y - (glyph.ymin as i32 + glyph.height as i32);

        for py in 0..glyph.height {
            for px in 0..glyph.width {
                let pixel_x = x_offset + (px as i32);
                let pixel_y = y_offset + (py as i32);

                let pixel_index = (py as usize) * (glyph.width as usize) + (px as usize);
                let byte_index = glyph.bitmap_offset as usize + (pixel_index / 2);
                let is_high_nibble = (pixel_index & 1) == 0;

                let byte = self.bitmap_data[byte_index];
                let alpha = if is_high_nibble {
                    (byte >> 4) & 0x0F
                } else {
                    byte & 0x0F
                };

                if alpha > 0 {
                    let pixel_color = if alpha >= 15 {
                        color
                    } else {
                        let bg_color = display.get_pixel(pixel_x as u16, pixel_y as u16);

                        blend_rgb565(color, bg_color, alpha)
                    };

                    display.set_pixel(pixel_x as u16, pixel_y as u16, pixel_color);
                }
            }
        }
    }
}

pub struct Text {
    pub content: String,
    pub x: u16,
    pub y: u16,
    pub color: Color,
}

impl Graphic for Text {
    fn draw(&self, display: &mut Display) -> Result<(), GraphicsError> {
        debug!(
            "Drawing text '{}' at ({}, {}) with color {:?}",
            self.content, self.x, self.y, self.color
        );

        let font = BinaryFont::new(FONT_BITMAP)?;
        let color = self.color.into();

        let mut cursor_x = self.x as i32;
        let cursor_y = self.y as i32;

        for ch in self.content.chars() {
            let glyph = font.find_glyph(ch)?;

            font.render_glyph(&glyph, cursor_x, cursor_y, color, display);

            cursor_x += glyph.advance_width as i32;
        }

        Ok(())
    }
}
