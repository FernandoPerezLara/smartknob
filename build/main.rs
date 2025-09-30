use std::path::Path;

use crate::fonts::FontBuilder;

mod fonts;

pub trait AssetBuilder {
    fn new(path: impl AsRef<Path>) -> Self;
    fn run(&self);
}

fn main() {
    let path = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    FontBuilder::new(Path::new(&path).join("fonts")).run();
}
