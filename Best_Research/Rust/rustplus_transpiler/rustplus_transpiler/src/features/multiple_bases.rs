use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct MultipleBasesFeature;

impl LanguageFeature for MultipleBasesFeature {
    fn name(&self) -> &'static str {
        return "multiple_bases";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.multiple_bases;
    }

    fn validate(&self, items: &[TopLevelItem], _context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            if let TopLevelItem::Class(declaration) = item {
                if declaration.bases.len() > 1 {
                    bail!(
                        "language feature 'multiple_bases' is disabled, but class '{}' has {} bases",
                        declaration.name,
                        declaration.bases.len()
                    );
                }
            }
        }

        return Ok(());
    }
}
