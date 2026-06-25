use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct RustPlusConfig {
    #[serde(default)]
    pub features: FeatureFlags,
}

impl Default for RustPlusConfig {
    fn default() -> Self {
        return Self {
            features: FeatureFlags::default(),
        };
    }
}

impl RustPlusConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
        let config = toml::from_str::<RustPlusConfig>(&source)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        return Ok(config);
    }

    pub fn load_optional(path: Option<&Path>) -> Result<Self> {
        match path {
            Some(path) => return Self::load_from_file(path),
            None => return Ok(Self::default()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FeatureFlags {
    pub classes: bool,
    pub interfaces: bool,
    pub abstract_classes: bool,
    pub multiple_bases: bool,
    pub visibility_modifiers: bool,
    pub this_receiver: bool,
    pub composition_bases: bool,
    pub generics: bool,
    pub csharp_variable_declarations: bool,
    pub new_expressions: bool,
    pub interface_object_sugar: bool,
    pub stack_heap_initializers: bool,
    pub preserve_stack_heap_methods: bool,
    pub attribute_passthrough: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        return Self {
            classes: true,
            interfaces: true,
            abstract_classes: true,
            multiple_bases: true,
            visibility_modifiers: true,
            this_receiver: true,
            composition_bases: true,
            generics: true,
            csharp_variable_declarations: false,
            new_expressions: false,
            interface_object_sugar: true,
            stack_heap_initializers: true,
            preserve_stack_heap_methods: false,
            attribute_passthrough: true,
        };
    }
}

impl FeatureFlags {
    pub fn set_by_name(&mut self, feature_name: &str, enabled: bool) -> Result<()> {
        match feature_name {
            "classes" | "class" => self.classes = enabled,
            "interfaces" | "interface" => self.interfaces = enabled,
            "abstract_classes" | "abstract-class" | "abstract" => self.abstract_classes = enabled,
            "multiple_bases" | "multiple-bases" => self.multiple_bases = enabled,
            "visibility_modifiers" | "visibility-modifiers" | "visibility" => self.visibility_modifiers = enabled,
            "this_receiver" | "this-receiver" | "this" => self.this_receiver = enabled,
            "composition_bases" | "composition-bases" | "composition" => self.composition_bases = enabled,
            "generics" | "generic" => self.generics = enabled,
            "csharp_variable_declarations" | "csharp-variable-declarations" | "csharp_vars" | "csharp-vars" => {
                self.csharp_variable_declarations = enabled
            }
            "new_expressions" | "new-expressions" | "new" => self.new_expressions = enabled,
            "interface_object_sugar" | "interface-object-sugar" | "interface-objects" | "trait-objects" => {
                self.interface_object_sugar = enabled
            }
            "stack_heap_initializers" | "stack-heap-initializers" | "stack_heap" | "stack-heap" => {
                self.stack_heap_initializers = enabled
            }
            "preserve_stack_heap_methods" | "preserve-stack-heap-methods" | "stack-heap-methods" => {
                self.preserve_stack_heap_methods = enabled
            }
            "attribute_passthrough" | "attribute-passthrough" | "attributes" | "attrs" => {
                self.attribute_passthrough = enabled
            }
            _ => anyhow::bail!("unknown language feature: {}", feature_name),
        }

        return Ok(());
    }
}
