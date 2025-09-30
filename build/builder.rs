use std::path::Path;

pub trait AssetBuilder {
    fn new(path: impl AsRef<Path>) -> Self;
    fn run(&self);
}
