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
use crate::transpiler::SemanticContext;
use anyhow::Result;

pub trait LanguageFeature {
    fn name(&self) -> &'static str;
    fn enabled(&self, flags: &FeatureFlags) -> bool;
    fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FeatureRegistry;

impl FeatureRegistry {
    pub fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
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
