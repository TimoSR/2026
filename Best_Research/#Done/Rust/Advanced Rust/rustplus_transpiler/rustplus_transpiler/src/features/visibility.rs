use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct VisibilityModifiersFeature;

impl LanguageFeature for VisibilityModifiersFeature {
    fn name(&self) -> &'static str {
        return "visibility_modifiers";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.visibility_modifiers;
    }

    fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            match item {
                TopLevelItem::Interface(declaration) | TopLevelItem::AbstractClass(declaration) | TopLevelItem::Class(declaration) => {
                    if declaration.visibility.is_some() {
                        bail!("language feature 'visibility_modifiers' is disabled, but '{}' has a visibility modifier", declaration.name);
                    }
                }
                TopLevelItem::Raw(_) => {}
            }

            if let TopLevelItem::Class(declaration) = item {
                let Some(body) = context.class_body(&declaration.name) else {
                    continue;
                };

                for field in &body.fields {
                    let value = field.source.trim();
                    if value.starts_with("private ") || value.starts_with("public ") || value.starts_with("pub ") {
                        bail!("language feature 'visibility_modifiers' is disabled, but class '{}' has a visible field", declaration.name);
                    }
                }

                for method in &body.methods {
                    let value = method.source.trim();
                    if value.starts_with("private ") || value.starts_with("public ") || value.starts_with("pub ") {
                        bail!("language feature 'visibility_modifiers' is disabled, but class '{}' has a visible method", declaration.name);
                    }
                }
            }
        }

        return Ok(());
    }
}
