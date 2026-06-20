use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::ProjectSymbols;
use anyhow::{bail, Result};

pub struct GenericsFeature;

impl LanguageFeature for GenericsFeature {
    fn name(&self) -> &'static str {
        return "generics";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.generics;
    }

    fn validate(&self, items: &[TopLevelItem], _context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            match item {
                TopLevelItem::Interface(declaration) | TopLevelItem::AbstractClass(declaration) | TopLevelItem::Class(declaration) => {
                    if declaration.has_generics() {
                        bail!("language feature 'generics' is disabled, but '{}' uses generics", declaration.name);
                    }
                    for base in &declaration.bases {
                        if base.contains('<') || base.contains('>') {
                            bail!("language feature 'generics' is disabled, but '{}' has generic base '{}'", declaration.name, base);
                        }
                    }
                }
                TopLevelItem::Raw(_) => {}
            }
        }

        return Ok(());
    }
}
