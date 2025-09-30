use std::path::{Path, PathBuf};

use crate::AssetBuilder;

pub struct FontBuilder {
    _path: PathBuf,
}

impl AssetBuilder for FontBuilder {
    fn new(path: impl AsRef<Path>) -> Self {
        Self {
            _path: path.as_ref().to_path_buf(),
        }
    }

    fn run(&self) {
        println!("cargo::rerun-if-changed=assets/fonts");
    }
}
