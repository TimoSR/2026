use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct CompositionBasesFeature;

impl LanguageFeature for CompositionBasesFeature {
    fn name(&self) -> &'static str {
        return "composition_bases";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.composition_bases;
    }

    fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            if let TopLevelItem::Class(declaration) = item {
                for base in &declaration.bases {
                    if context.is_composed_base_class(base) {
                        bail!(
                            "language feature 'composition_bases' is disabled, but class '{}' composes base class '{}'",
                            declaration.name,
                            base
                        );
                    }
                }
            }
        }

        return Ok(());
    }
}
