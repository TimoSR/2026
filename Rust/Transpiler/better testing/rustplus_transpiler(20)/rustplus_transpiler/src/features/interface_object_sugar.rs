use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::transpiler::ProjectSymbols;
use anyhow::Result;

pub struct InterfaceObjectSugarFeature;

impl LanguageFeature for InterfaceObjectSugarFeature {
    fn name(&self) -> &'static str {
        return "interface_object_sugar";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.interface_object_sugar;
    }

    fn validate(&self, _items: &[TopLevelItem], _context: &ProjectSymbols, _flags: &FeatureFlags) -> Result<()> {
        // Validation for this feature happens inside the C# variable declaration rewrite because
        // it needs the parsed initializer and the declared type together.
        return Ok(());
    }
}
