use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::csharp_variable_declarations::rewrite_new_expressions;
use crate::features::LanguageFeature;
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct NewExpressionsFeature;

impl LanguageFeature for NewExpressionsFeature {
    fn name(&self) -> &'static str {
        return "new_expressions";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.new_expressions;
    }

    fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        let enabled_flags = FeatureFlags {
            new_expressions: true,
            ..flags.clone()
        };

        for item in items {
            match item {
                TopLevelItem::Raw(source) => {
                    if rewrite_new_expressions(source, context, &enabled_flags)? != source.as_str() {
                        bail!("language feature 'new_expressions' is disabled, but C#-style 'new Type(...)' was used");
                    }
                }
                TopLevelItem::Interface(declaration)
                | TopLevelItem::AbstractClass(declaration)
                | TopLevelItem::Class(declaration) => {
                    if rewrite_new_expressions(&declaration.body, context, &enabled_flags)? != declaration.body.as_str() {
                        bail!(
                            "language feature 'new_expressions' is disabled, but '{}' uses C#-style 'new Type(...)'",
                            declaration.name
                        );
                    }
                }
            }
        }

        return Ok(());
    }
}
