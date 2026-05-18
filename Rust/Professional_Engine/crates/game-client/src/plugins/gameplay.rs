use engine_core::{AppContext, EngineEvent, Plugin, Result};

#[derive(Default)]
pub(crate) struct GameplayPlugin {
    x: f32,
    y: f32,
}

impl Plugin for GameplayPlugin {
    fn name(&self) -> &'static str {
        "gameplay"
    }

    fn on_start(&mut self, _ctx: &mut AppContext) -> Result<()> {
        println!("[gameplay] initialized");
        Ok(())
    }

    fn on_update(&mut self, ctx: &mut AppContext) -> Result<()> {
        self.x += 1.5;
        self.y = (ctx.frame() as f32).sin();

        ctx.emit(EngineEvent::DrawSprite {
            sprite: "hero".to_owned(),
            x: self.x,
            y: self.y,
        });

        if ctx.frame() % 2 == 0 {
            ctx.emit(EngineEvent::PlaySound {
                clip: "footstep".to_owned(),
            });
        }

        ctx.emit(EngineEvent::Log {
            source: "gameplay",
            message: format!("tick {} at dt={:.4}s", ctx.frame(), ctx.delta_seconds()),
        });

        Ok(())
    }

    fn on_stop(&mut self, _ctx: &mut AppContext) -> Result<()> {
        println!("[gameplay] shutdown");
        Ok(())
    }
}
