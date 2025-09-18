mod views;

use crate::peripherals::display::Display;
use alloc::boxed::Box;
use alloc::vec::Vec;
pub use views::{LightView, View};

#[derive(Default)]
pub struct ViewManager {
    views: Vec<Box<dyn View>>,
}

impl ViewManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, view: Box<dyn View>) {
        self.views.push(view);
    }

    pub fn select(&self, index: usize, display: &mut Display) {
        if let Some(view) = self.views.get(index) {
            view.render(display);
        }
    }

    pub fn len(&self) -> usize {
        self.views.len()
    }

    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }
}
