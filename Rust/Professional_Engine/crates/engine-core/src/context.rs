use crate::EngineEvent;

#[derive(Debug)]
pub struct AppContext {
    frame: u64,
    delta_seconds: f32,
    events: Vec<EngineEvent>,
}

impl AppContext {
    #[must_use]
    pub fn new(delta_seconds: f32) -> Self {
        Self {
            frame: 0,
            delta_seconds,
            events: Vec::new(),
        }
    }

    pub fn begin_frame(&mut self, frame: u64) {
        self.frame = frame;
        self.events.clear();
    }

    #[must_use]
    pub fn frame(&self) -> u64 {
        self.frame
    }

    #[must_use]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    pub fn emit(&mut self, event: EngineEvent) {
        self.events.push(event);
    }

    #[must_use]
    pub fn events(&self) -> &[EngineEvent] {
        &self.events
    }
}
