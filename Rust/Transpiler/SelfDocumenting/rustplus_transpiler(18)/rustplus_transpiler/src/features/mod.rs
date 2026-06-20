pub mod abstract_class;
pub mod attribute_passthrough;
pub mod class;
pub mod composition_bases;
pub mod csharp_variable_declarations;
pub mod generics;
pub mod interface;
pub mod interface_object_sugar;
pub mod multiple_bases;
pub mod new_expressions;
pub mod stack_heap_initializers;
pub mod this_receiver;
pub mod visibility;

use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::transpiler::ProjectSymbols;
use anyhow::Result;

/// One optional Rust Plus language capability.
///
/// A feature is responsible for answering two simple questions:
///
/// - is this feature enabled in `rustplus.toml`?
/// - is the current source valid under that setting?
///
/// Actual Rust emission remains in the generator/transform layer unless the
/// feature owns a small local rewrite helper.
pub trait LanguageFeature {
    fn name(&self) -> &'static str;
    fn enabled(&self, flags: &FeatureFlags) -> bool;
    fn validate(&self, items: &[TopLevelItem], context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()>;
}

/// Ordered list of enabled language features.
///
/// Keep this explicit. It is easier to read and debug than a macro or boxed
/// dynamic registry, and it shows exactly which feature validates before the
/// next one.
#[derive(Debug, Default, Clone, Copy)]
pub struct FeaturePipeline;

impl FeaturePipeline {
    pub fn validate(&self, items: &[TopLevelItem], context: &ProjectSymbols, flags: &FeatureFlags) -> Result<()> {
        class::ClassFeature.validate(items, context, flags)?;
        interface::InterfaceFeature.validate(items, context, flags)?;
        abstract_class::AbstractClassFeature.validate(items, context, flags)?;
        multiple_bases::MultipleBasesFeature.validate(items, context, flags)?;
        visibility::VisibilityModifiersFeature.validate(items, context, flags)?;
        this_receiver::ThisReceiverFeature.validate(items, context, flags)?;
        composition_bases::CompositionBasesFeature.validate(items, context, flags)?;
        generics::GenericsFeature.validate(items, context, flags)?;
        csharp_variable_declarations::CSharpVariableDeclarationsFeature.validate(items, context, flags)?;
        new_expressions::NewExpressionsFeature.validate(items, context, flags)?;
        interface_object_sugar::InterfaceObjectSugarFeature.validate(items, context, flags)?;
        stack_heap_initializers::StackHeapInitializersFeature.validate(items, context, flags)?;
        attribute_passthrough::AttributePassthroughFeature.validate(items, context, flags)?;

        return Ok(());
    }

    pub fn names(&self) -> &'static [&'static str] {
        return &[
            "classes",
            "interfaces",
            "abstract_classes",
            "multiple_bases",
            "visibility_modifiers",
            "this_receiver",
            "composition_bases",
            "generics",
            "csharp_variable_declarations",
            "new_expressions",
            "interface_object_sugar",
            "stack_heap_initializers",
            "preserve_stack_heap_methods",
            "attribute_passthrough",
        ];
    }
}

/// Backwards-compatible name used by older code. Prefer `FeaturePipeline` in new
/// code because it describes the concept more directly.
pub type FeatureRegistry = FeaturePipeline;
