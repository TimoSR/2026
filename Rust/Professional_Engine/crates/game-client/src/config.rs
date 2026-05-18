#[derive(Debug, Clone, Copy)]
pub struct GameConfig {
    pub max_frames: u64,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { max_frames: 6 }
    }
}
