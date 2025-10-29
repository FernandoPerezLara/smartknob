pub mod fonts;

use std::path::Path;

pub trait AssetBuilder {
    fn new(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Self;
    fn run(&self);
}
