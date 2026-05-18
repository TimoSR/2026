#[derive(Debug, Clone)]
pub enum EngineEvent {
    DrawSprite {
        sprite: String,
        x: f32,
        y: f32,
    },
    PlaySound {
        clip: String,
    },
    Log {
        source: &'static str,
        message: String,
    },
}
