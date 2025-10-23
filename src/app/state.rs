#[derive(Default, Clone, Copy)]
pub struct AppState {
    pub angle: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
