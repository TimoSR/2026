#[derive(Debug, Clone, Copy)]
pub struct RuntimeConfig {
    pub max_frames: u64,
    pub delta_seconds: f32,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_frames: 6,
            delta_seconds: 1.0 / 60.0,
        }
    }
}
