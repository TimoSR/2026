use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::parser::contains_attribute_syntax;
use crate::transpiler::ProjectSymbols;
use anyhow::{bail, Result};

pub struct AttributePassthroughFeature;

impl LanguageFeature for AttributePassthroughFeature {
    fn name(&self) -> &'static str {
        return "attribute_passthrough";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.attribute_passthrough;
    }

    fn validate(&self, items: &[TopLevelItem], context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            match item {
                TopLevelItem::Raw(source) => {
                    if contains_attribute_syntax(source) {
                        bail!("language feature 'attribute_passthrough' is disabled, but Rust attribute syntax was used");
                    }
                }
                TopLevelItem::Interface(declaration)
                | TopLevelItem::AbstractClass(declaration)
                | TopLevelItem::Class(declaration) => {
                    if !declaration.attributes.is_empty() || contains_attribute_syntax(&declaration.body) {
                        bail!(
                            "language feature 'attribute_passthrough' is disabled, but '{}' uses Rust attributes",
                            declaration.name
                        );
                    }
                }
            }
        }

        for body in context.class_bodies.values() {
            for field in &body.fields {
                if !field.attributes.is_empty() {
                    bail!("language feature 'attribute_passthrough' is disabled, but a class field uses Rust attributes");
                }
            }

            for method in &body.methods {
                if !method.attributes.is_empty() {
                    bail!("language feature 'attribute_passthrough' is disabled, but a class method uses Rust attributes");
                }
            }
        }

        return Ok(());
    }
}
