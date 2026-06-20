use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::ProjectSymbols;
use anyhow::{bail, Result};

pub struct InterfaceFeature;

impl LanguageFeature for InterfaceFeature {
    fn name(&self) -> &'static str {
        return "interfaces";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.interfaces;
    }

    fn validate(&self, items: &[TopLevelItem], _context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            if let TopLevelItem::Interface(declaration) = item {
                bail!("language feature 'interfaces' is disabled, but interface '{}' was used", declaration.name);
            }
        }

        return Ok(());
    }
}
