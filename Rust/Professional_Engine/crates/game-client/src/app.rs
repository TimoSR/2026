use engine_audio::AudioPlugin;
use engine_core::Result;
use engine_render::RenderPlugin;
use engine_runtime::{Runtime, RuntimeConfig};

use crate::config::GameConfig;
use crate::plugins::GameplayPlugin;

pub struct GameApplication {
    runtime: Runtime,
}

impl GameApplication {
    #[must_use]
    pub fn new(config: GameConfig) -> Self {
        let mut runtime = Runtime::new(RuntimeConfig {
            max_frames: config.max_frames,
            ..RuntimeConfig::default()
        });

        runtime
            .add_plugin(GameplayPlugin::default())
            .add_plugin(RenderPlugin::new())
            .add_plugin(AudioPlugin::new());

        Self { runtime }
    }

    pub fn run(self) -> Result<()> {
        self.runtime.run()
    }
}

pub fn run(config: GameConfig) -> Result<()> {
    GameApplication::new(config).run()
}
