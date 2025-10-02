use std::{
    collections::HashMap,
    fs::{File, create_dir_all, read, read_to_string},
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
        }
    }
}
