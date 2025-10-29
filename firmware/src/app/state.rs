#[derive(Default)]
pub struct AppState {
    pub angle: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
