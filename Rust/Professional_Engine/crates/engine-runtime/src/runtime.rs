use engine_core::{AppContext, Plugin, Result};

use crate::RuntimeConfig;

type BoxedPlugin = Box<dyn Plugin>;

pub struct Runtime {
    config: RuntimeConfig,
    plugins: Vec<BoxedPlugin>,
}

impl Runtime {
    #[must_use]
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<P>(&mut self, plugin: P) -> &mut Self
    where
        P: Plugin + 'static,
    {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn run(mut self) -> Result<()> {
        let mut ctx = AppContext::new(self.config.delta_seconds);
        Self::startup_plugins(&mut self.plugins, &mut ctx)?;

        for frame in 0..self.config.max_frames {
            ctx.begin_frame(frame);
            Self::update_plugins(&mut self.plugins, &mut ctx, frame)?;
        }

        Self::shutdown_plugins(&mut self.plugins, &mut ctx)
    }

    fn startup_plugins(plugins: &mut [BoxedPlugin], ctx: &mut AppContext) -> Result<()> {
        for plugin in plugins {
            let plugin_name = plugin.name();
            if let Err(err) = plugin.on_start(ctx) {
                return Err(
                    err.with_context(format!("plugin `{plugin_name}` failed during startup"))
                );
            }
        }

        Ok(())
    }

    fn update_plugins(plugins: &mut [BoxedPlugin], ctx: &mut AppContext, frame: u64) -> Result<()> {
        for plugin in plugins {
            let plugin_name = plugin.name();
            if let Err(err) = plugin.on_update(ctx) {
                return Err(err.with_context(format!(
                    "plugin `{plugin_name}` failed during update on frame {frame}"
                )));
            }
        }

        Ok(())
    }

    fn shutdown_plugins(plugins: &mut [BoxedPlugin], ctx: &mut AppContext) -> Result<()> {
        for plugin in plugins.iter_mut().rev() {
            let plugin_name = plugin.name();
            if let Err(err) = plugin.on_stop(ctx) {
                return Err(
                    err.with_context(format!("plugin `{plugin_name}` failed during shutdown"))
                );
            }
        }

        Ok(())
    }
}
