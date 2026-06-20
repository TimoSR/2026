use crate::ast::{ClassBody, DeclarationKind, ParameterDeclaration, TopLevelItem};
use crate::codegen::RustCodeGenerator;
use crate::config::RustPlusConfig;
use crate::features::FeaturePipeline;
use crate::parser::{extract_method_parameters, extract_methods, parse_class_body, parse_top_level_items};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Project-wide language facts discovered from `.rp` files.
///
/// This is intentionally not a full Rust type checker. It only records the facts
/// the Rust Plus surface syntax needs before lowering to Rust: which names are
/// classes, which names are interfaces/abstract classes, which methods belong to
/// each interface, and which constructors/default initializers exist.
///
/// Older modules may still import this type through the `SemanticContext` alias.
#[derive(Debug, Clone, Default)]
pub struct ProjectSymbols {
    pub interface_methods: HashMap<String, HashSet<String>>,
    pub class_names: HashSet<String>,
    pub class_bodies: HashMap<String, ClassBody>,
    pub declaration_kinds: HashMap<String, DeclarationKind>,
    pub constructor_parameters: HashMap<String, Vec<ParameterDeclaration>>,
    pub default_initializers: HashSet<String>,
}

impl ProjectSymbols {
    pub fn from_items(items: &[TopLevelItem]) -> Result<Self> {
        let mut context = Self::default();

        for item in items {
            match item {
                TopLevelItem::Class(declaration) => {
                    context.class_names.insert(declaration.name.clone());
                    context
                        .declaration_kinds
                        .insert(declaration.name.clone(), DeclarationKind::Class);

                    let class_body = parse_class_body(&declaration.body)?;

                    if let Some(constructor) = class_body.methods.iter().find(|method| method.name == "new") {
                        context.constructor_parameters.insert(
                            declaration.name.clone(),
                            extract_method_parameters(&constructor.source)?,
                        );
                    }

                    if class_body.methods.iter().any(|method| method.name == "default") {
                        context.default_initializers.insert(declaration.name.clone());
                    }

                    context.class_bodies.insert(declaration.name.clone(), class_body);
                }
                TopLevelItem::Interface(declaration) => {
                    context
                        .declaration_kinds
                        .insert(declaration.name.clone(), DeclarationKind::Interface);
                    context.interface_methods.insert(
                        declaration.name.clone(),
                        extract_methods(&declaration.body)?
                            .into_iter()
                            .map(|method| method.name)
                            .collect::<HashSet<String>>(),
                    );
                }
                TopLevelItem::AbstractClass(declaration) => {
                    context
                        .declaration_kinds
                        .insert(declaration.name.clone(), DeclarationKind::AbstractClass);
                    context.interface_methods.insert(
                        declaration.name.clone(),
                        extract_methods(&declaration.body)?
                            .into_iter()
                            .map(|method| method.name)
                            .collect::<HashSet<String>>(),
                    );
                }
                TopLevelItem::Raw(source) => {
                    context.collect_default_impls_from_raw(source);
                }
            }
        }

        return Ok(context);
    }

    pub fn from_sources<'a, I>(sources: I) -> Result<Self>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut all_items = Vec::new();

        for source in sources {
            all_items.extend(parse_top_level_items(source)?);
        }

        return Self::from_items(&all_items);
    }

    pub fn class_body(&self, class_name: &str) -> Option<&ClassBody> {
        return self.class_bodies.get(class_name);
    }

    pub fn constructor_parameters(&self, class_name: &str) -> Option<&[ParameterDeclaration]> {
        let key = terminal_type_name(type_name_without_generics(class_name));
        return self.constructor_parameters.get(key).map(Vec::as_slice);
    }

    pub fn is_constructible_type(&self, type_name: &str) -> bool {
        let key = terminal_type_name(type_name_without_generics(type_name));
        return self.class_names.contains(key) || self.constructor_parameters.contains_key(key) || self.default_initializers.contains(key);
    }

    pub fn has_default_initializer(&self, type_name: &str) -> bool {
        let key = terminal_type_name(type_name_without_generics(type_name));
        return self.default_initializers.contains(key);
    }

    pub fn interface_method_names(&self, base_name: &str) -> Option<&HashSet<String>> {
        return self.interface_methods.get(type_name_without_generics(base_name));
    }

    fn collect_default_impls_from_raw(&mut self, source: &str) {
        for type_name in find_default_impl_type_names(source) {
            self.default_initializers.insert(type_name);
        }
    }

    pub fn is_trait_target(&self, base_name: &str) -> bool {
        let type_name = type_name_without_generics(base_name);

        if self.interface_methods.contains_key(type_name) {
            return true;
        }

        if self.class_names.contains(type_name) {
            return false;
        }

        return true;
    }

    pub fn is_composed_base_class(&self, base_name: &str) -> bool {
        let type_name = type_name_without_generics(base_name);
        return self.class_names.contains(type_name) && !self.interface_methods.contains_key(type_name);
    }

    pub fn is_known_or_likely_interface(&self, type_name: &str) -> bool {
        let base_name = type_name_without_generics(type_name);

        if self.interface_methods.contains_key(base_name) {
            return true;
        }

        if self.class_names.contains(base_name) {
            return false;
        }

        return looks_like_csharp_interface_name(base_name);
    }
}

/// Backwards-compatible alias used by older modules. Prefer `ProjectSymbols`
/// in new code because it describes the concept in product language.
pub type SemanticContext = ProjectSymbols;

fn type_name_without_generics(value: &str) -> &str {
    return value.split('<').next().unwrap_or(value).trim();
}

fn terminal_type_name(value: &str) -> &str {
    return value.rsplit("::").next().unwrap_or(value).trim();
}

fn find_default_impl_type_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut cursor = 0usize;

    while let Some(relative_index) = source[cursor..].find("impl Default for ") {
        let start = cursor + relative_index + "impl Default for ".len();
        let mut end = start;

        while end < source.len() {
            let character = source.as_bytes()[end] as char;

            if character.is_ascii_alphanumeric() || character == '_' || character == ':' {
                end += 1;
                continue;
            }

            break;
        }

        let type_name = source[start..end].trim();

        if !type_name.is_empty() {
            names.push(terminal_type_name(type_name).to_string());
        }

        cursor = end;
    }

    return names;
}

fn looks_like_csharp_interface_name(value: &str) -> bool {
    let mut characters = value.chars();

    if characters.next() != Some('I') {
        return false;
    }

    return characters.next().is_some_and(|character| character.is_ascii_uppercase());
}

#[derive(Debug, Clone)]
pub struct Transpiler {
    source: String,
    config: RustPlusConfig,
}

impl Transpiler {
    pub fn new(source: String) -> Self {
        return Self {
            source,
            config: RustPlusConfig::default(),
        };
    }

    pub fn with_config(source: String, config: RustPlusConfig) -> Self {
        return Self { source, config };
    }

    pub fn transpile(&self) -> Result<String> {
        let items = parse_top_level_items(&self.source)?;
        let context = SemanticContext::from_items(&items)?;
        return self.transpile_items_with_context(&items, &context);
    }

    pub fn transpile_with_project_context(&self, context: &SemanticContext) -> Result<String> {
        let items = parse_top_level_items(&self.source)?;
        return self.transpile_items_with_context(&items, context);
    }

    fn transpile_items_with_context(&self, items: &[TopLevelItem], context: &SemanticContext) -> Result<String> {
        let feature_pipeline = FeaturePipeline::default();
        feature_pipeline.validate(items, context, &self.config.features)?;

        let codegen = RustCodeGenerator::new(&self.config.features, context);
        let mut output = String::with_capacity(self.source.len() + self.source.len() / 8);

        for item in items {
            codegen.emit_item_into(item, &mut output)?;

            if !output.ends_with('\n') {
                output.push('\n');
            }
        }

        return Ok(output);
    }
}

#[cfg(test)]
mod attribute_passthrough_tests {
    use super::*;
    use crate::config::RustPlusConfig;

    #[test]
    fn emits_outer_attributes_on_generated_class_fields_and_methods() {
        let source = r#"
#[derive(Debug, Clone, PartialEq, Eq)]
pub class Article
{
    #[doc = "headline"]
    pub headline: String;

    #[inline]
    pub fn new(headline: &str) -> Self
    {
        return Self
        {
            headline: headline.to_string(),
        };
    }
}
"#;

        let rust = Transpiler::new(source.to_string()).transpile().expect("transpile should succeed");

        assert!(rust.contains("#[derive(Debug, Clone, PartialEq, Eq)]\npub struct Article"));
        assert!(rust.contains("    #[doc = \"headline\"]\n    pub headline: String,"));
        assert!(rust.contains("    #[inline]\n    pub fn new(headline: &str) -> Self"));
    }

    #[test]
    fn errors_when_attributes_are_disabled() {
        let source = r#"
#[derive(Debug)]
class Article
{
    headline: String;
}
"#;
        let mut config = RustPlusConfig::default();
        config.features.attribute_passthrough = false;

        let error = Transpiler::with_config(source.to_string(), config)
            .transpile()
            .expect_err("attributes should be rejected when the feature is disabled");

        assert!(error.to_string().contains("attribute_passthrough"));
    }
}
