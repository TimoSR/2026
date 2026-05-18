use engine_core::{AppContext, EngineEvent, Plugin, Result};

#[derive(Default)]
pub struct AudioPlugin {
    played_clips: u64,
}

impl AudioPlugin {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Plugin for AudioPlugin {
    fn name(&self) -> &'static str {
        "engine-audio"
    }

    fn on_start(&mut self, _ctx: &mut AppContext) -> Result<()> {
        println!("[audio] initialized");
        Ok(())
    }

    fn on_update(&mut self, ctx: &mut AppContext) -> Result<()> {
        let mut frame_had_audio = false;

        for event in ctx.events() {
            if let EngineEvent::PlaySound { clip } = event {
                frame_had_audio = true;
                self.played_clips += 1;
                println!("[audio] frame {} play `{clip}`", ctx.frame());
            }
        }

        if !frame_had_audio {
            println!("[audio] frame {} idle", ctx.frame());
        }

        Ok(())
    }

    fn on_stop(&mut self, _ctx: &mut AppContext) -> Result<()> {
        println!("[audio] shutdown after {} clips", self.played_clips);
        Ok(())
    }
}
