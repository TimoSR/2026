use engine_core::{AppContext, EngineEvent, Plugin, Result};

#[derive(Default)]
pub struct RenderPlugin {
    rendered_frames: u64,
}

impl RenderPlugin {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Plugin for RenderPlugin {
    fn name(&self) -> &'static str {
        "engine-render"
    }

    fn on_start(&mut self, _ctx: &mut AppContext) -> Result<()> {
        println!("[render] initialized");
        Ok(())
    }

    fn on_update(&mut self, ctx: &mut AppContext) -> Result<()> {
        let mut draw_calls = 0_u32;

        for event in ctx.events() {
            match event {
                EngineEvent::DrawSprite { sprite, x, y } => {
                    draw_calls += 1;
                    println!(
                        "[render] frame {} draw `{}` at ({:.1}, {:.1})",
                        ctx.frame(),
                        sprite,
                        x,
                        y
                    );
                }
                EngineEvent::Log { source, message } => {
                    println!(
                        "[render] frame {} log from {source}: {message}",
                        ctx.frame()
                    );
                }
                EngineEvent::PlaySound { .. } => {}
            }
        }

        if draw_calls == 0 {
            println!("[render] frame {} no draw calls", ctx.frame());
        }

        self.rendered_frames += 1;
        Ok(())
    }

    fn on_stop(&mut self, _ctx: &mut AppContext) -> Result<()> {
        println!("[render] shutdown after {} frames", self.rendered_frames);
        Ok(())
    }
}
