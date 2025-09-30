mod builder;
mod fonts;

use std::path::Path;

use crate::{builder::AssetBuilder, fonts::FontBuilder};

fn main() {
    let path = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    FontBuilder::new(Path::new(&path).join("fonts")).run();
}
