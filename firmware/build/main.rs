mod assets;

use std::path::PathBuf;

use assets::{AssetBuilder, fonts::FontBuilder};

fn main() {
    let input_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let output_path = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));

    FontBuilder::new(input_path.join("assets/fonts"), output_path.join("fonts")).run();
}
