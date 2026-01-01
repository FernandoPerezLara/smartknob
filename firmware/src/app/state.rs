#[derive(Default)]
pub struct AppState {
    pub position: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
