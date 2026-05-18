use crate::{AppContext, Result};

pub trait Plugin {
    fn name(&self) -> &'static str;

    fn on_start(&mut self, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn on_update(&mut self, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn on_stop(&mut self, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }
}
