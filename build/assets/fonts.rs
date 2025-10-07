use std::{
    collections::HashMap,
    fs::{File, create_dir_all, read, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use fontdue::{Font, FontSettings};
use serde::Deserialize;

use crate::AssetBuilder;

const CONFIG_FILE: &str = "fonts.toml";

#[derive(Deserialize, Debug)]
struct FontConfigs {
    fonts: HashMap<String, FontConfig>,
}

#[derive(Deserialize, Debug)]
struct FontConfig {
    file: String,
    size: u32,
    charset: String,
}

pub struct FontBuilder {
    input_path: PathBuf,
    output_path: PathBuf,
}

impl AssetBuilder for FontBuilder {
    fn new(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Self {
        Self {
            input_path: input_path.as_ref().to_path_buf(),
            output_path: output_path.as_ref().to_path_buf(),
        }
    }

    fn run(&self) {
        println!("cargo::rerun-if-changed={}", self.input_path.display());

        create_dir_all(&self.output_path).expect("Failed to create output directory");

        let config_raw = read_to_string(self.input_path.join(CONFIG_FILE))
            .expect("Failed to read font config file");
        let config: FontConfigs =
            toml::from_str(&config_raw).expect("Failed to parse font config file");

        for (name, config) in config.fonts {
            let font_raw =
                read(self.input_path.join(&config.file)).expect("Failed to read font file");
            let font = Font::from_bytes(font_raw, FontSettings::default())
                .expect("Failed to load font from bytes");

            let mut output_path = self.output_path.join(&name);
            output_path.set_extension("bin");
            let mut output_file = File::create(&output_path).expect("Failed to create output file");

            // Binary font file format:
            //
            // | Section        | Size                | Description                          |
            // |----------------|---------------------|--------------------------------------|
            // | Header         | 8 bytes             | char_count (u32), max_baseline (u32) |
            // | Char Map       | char_count × 2      | Sorted u16 char codes (for lookup)   |
            // | Glyph Metadata | char_count × 6      | width, height, xmin, ymin, offset    |
            // | Bitmap Data    | Variable            | 4-bit antialiased pixels (packed)    |
            //
            // All multi-byte values are little-endian.
            // Bitmap: 2 pixels/byte, first pixel in high nibble.

            let mut chars: Vec<char> = config.charset.chars().collect();
            chars.sort();
            chars.dedup();

            let max_baseline = chars
                .iter()
                .map(|&ch| font.metrics(ch, config.size as f32).ymin.abs())
                .max()
                .unwrap_or(0) as u32;

            let mut char_map: Vec<u16> = Vec::new();
            let mut glyph_metadata: Vec<u8> = Vec::new();
            let mut bitmap_data: Vec<u8> = Vec::new();

            for &ch in &chars {
                char_map.push(ch as u16);

                let (metrics, bitmap) = font.rasterize(ch, config.size as f32);

                glyph_metadata.push(metrics.width as u8);
                glyph_metadata.push(metrics.height as u8);
                glyph_metadata.push(metrics.xmin as i8 as u8);
                glyph_metadata.push(metrics.ymin as i8 as u8);

                let offset = bitmap_data.len() as u16;
                glyph_metadata.extend_from_slice(&offset.to_le_bytes());

                let mut packed_bytes = Vec::new();
                let mut current_byte = 0u8;
                let mut pixel_count = 0;

                for alpha in bitmap {
                    let level = (alpha >> 4) & 0x0F;
                    current_byte |= level << (4 - (pixel_count % 2) * 4);
                    pixel_count += 1;

                    if pixel_count % 2 == 0 {
                        packed_bytes.push(current_byte);
                        current_byte = 0;
                    }
                }

                if pixel_count % 2 != 0 {
                    packed_bytes.push(current_byte);
                }

                bitmap_data.extend(packed_bytes);
            }

            output_file
                .write_all(&(chars.len() as u32).to_le_bytes())
                .expect("Failed to write character count");
            output_file
                .write_all(&max_baseline.to_le_bytes())
                .expect("Failed to write max baseline");

            let char_map_bytes: Vec<u8> = char_map.iter().flat_map(|&c| c.to_le_bytes()).collect();
            output_file
                .write_all(&char_map_bytes)
                .expect("Failed to write char map");

            output_file
                .write_all(&glyph_metadata)
                .expect("Failed to write glyph metadata");
            output_file
                .write_all(&bitmap_data)
                .expect("Failed to write bitmap data");
        }
    }
}
