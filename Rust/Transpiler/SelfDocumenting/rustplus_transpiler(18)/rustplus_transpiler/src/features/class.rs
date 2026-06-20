use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::ProjectSymbols;
use anyhow::{bail, Result};

pub struct ClassFeature;

impl LanguageFeature for ClassFeature {
    fn name(&self) -> &'static str {
        return "classes";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.classes;
    }

    fn validate(&self, items: &[TopLevelItem], _context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            if let TopLevelItem::Class(declaration) = item {
                bail!("language feature 'classes' is disabled, but class '{}' was used", declaration.name);
            }
        }

        return Ok(());
    }
}
