use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::ProjectSymbols;
use anyhow::{bail, Result};

pub struct AbstractClassFeature;

impl LanguageFeature for AbstractClassFeature {
    fn name(&self) -> &'static str {
        return "abstract_classes";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.abstract_classes;
    }

    fn validate(&self, items: &[TopLevelItem], _context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            if let TopLevelItem::AbstractClass(declaration) = item {
                bail!(
                    "language feature 'abstract_classes' is disabled, but abstract class '{}' was used",
                    declaration.name
                );
            }
        }

        return Ok(());
    }
}
