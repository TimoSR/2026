use crate::ast::{ClassBody, FieldDeclaration, MethodDeclaration, TopLevelItem, TypeDeclaration};
use crate::config::FeatureFlags;
use crate::features::csharp_variable_declarations::{rewrite_csharp_variable_declarations, rewrite_new_expressions};
use crate::features::stack_heap_initializers::rewrite_stack_heap_initializers;
use crate::features::this_receiver::rewrite_this_keyword;
use crate::parser::extract_method_parameters;
use crate::transpiler::ProjectSymbols;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct RustCodeGenerator<'a> {
    flags: &'a FeatureFlags,
    context: &'a ProjectSymbols,
}

impl<'a> RustCodeGenerator<'a> {
    pub fn new(flags: &'a FeatureFlags, context: &'a ProjectSymbols) -> Self {
        return Self { flags, context };
    }

    pub fn emit_item(&self, item: &TopLevelItem) -> Result<String> {
        let mut output = String::new();
        self.emit_item_into(item, &mut output)?;
        return Ok(output);
    }

    pub fn emit_item_into(&self, item: &TopLevelItem, output: &mut String) -> Result<()> {
        match item {
            TopLevelItem::Raw(raw) => output.push_str(&self.apply_statement_level_features(raw)?),
            TopLevelItem::Interface(declaration) | TopLevelItem::AbstractClass(declaration) => {
                self.emit_trait_into(declaration, output)?;
            }
            TopLevelItem::Class(declaration) => {
                self.emit_class_into(declaration, output)?;
            }
        }

        return Ok(());
    }

    fn emit_trait_into(&self, declaration: &TypeDeclaration, output: &mut String) -> Result<()> {
        let visibility = declaration.visibility.as_deref().unwrap_or("");
        let prefix = if visibility.is_empty() { "" } else { "pub " };
        let body = self.apply_statement_level_features(&rewrite_this_keyword(declaration.body.trim(), self.flags.this_receiver))?;

        emit_attributes_into(&declaration.attributes, output, "");
        output.push_str(prefix);
        output.push_str("trait ");
        output.push_str(&declaration.name);
        output.push_str(&declaration.generics_definition);
        output.push_str("\n{\n");
        output.push_str(&body);
        output.push_str("\n}\n");
        return Ok(());
    }

    fn emit_class_into(&self, declaration: &TypeDeclaration, output: &mut String) -> Result<()> {
        let class_body = self
            .context
            .class_body(&declaration.name)
            .ok_or_else(|| anyhow!("missing parsed body for class '{}'", declaration.name))?;
        let visibility = declaration.visibility.as_deref().unwrap_or("");
        let prefix = if visibility.is_empty() { "" } else { "pub " };

        emit_attributes_into(&declaration.attributes, output, "");
        output.push_str(prefix);
        output.push_str("struct ");
        output.push_str(&declaration.name);
        output.push_str(&declaration.generics_definition);
        output.push_str("\n{\n");

        for field in &class_body.fields {
            emit_attributes_into(&field.attributes, output, "    ");
            output.push_str("    ");
            output.push_str(&self.normalize_field(field)?);
            output.push_str(",\n");
        }

        for base_class in declaration
            .bases
            .iter()
            .filter(|base_name| self.context.is_composed_base_class(base_name))
        {
            output.push_str("    ");
            output.push_str(&to_snake_case(base_class));
            output.push_str(": ");
            output.push_str(base_class);
            output.push_str(",\n");
        }

        output.push_str("}\n");
        self.emit_class_impl_blocks(declaration, class_body, output)?;

        return Ok(());
    }

    fn emit_class_impl_blocks(
        &self,
        declaration: &TypeDeclaration,
        class_body: &ClassBody,
        output: &mut String,
    ) -> Result<()> {
        let trait_targets = declaration
            .bases
            .iter()
            .filter(|base_name| self.context.is_trait_target(base_name))
            .map(String::as_str)
            .collect::<Vec<&str>>();
        let mut trait_methods: HashMap<&str, Vec<&MethodDeclaration>> = HashMap::new();
        let mut inherent_methods: Vec<&MethodDeclaration> = Vec::new();

        for method in &class_body.methods {
            let target_trait = trait_targets.iter().copied().find(|interface_name| {
                self.context
                    .interface_method_names(*interface_name)
                    .is_some_and(|methods| methods.contains(&method.name))
            });

            match target_trait {
                Some(interface_name) => trait_methods.entry(interface_name).or_default().push(method),
                None => inherent_methods.push(method),
            }
        }

        if !inherent_methods.is_empty() {
            output.push_str("\nimpl");
            output.push_str(&declaration.generics_definition);
            output.push(' ');
            output.push_str(&declaration.name);
            output.push_str(&declaration.generics_usage);
            output.push_str("\n{\n");

            let constructor_method = inherent_methods.iter().copied().find(|method| method.name == "new");

            for method in inherent_methods {
                emit_attributes_into(&method.attributes, output, "    ");
                let normalized_method = self.normalize_inherent_method(method)?;
                emit_indented_source_into(normalized_method.trim(), output, "    ");
                output.push_str("\n\n");
            }

            if self.flags.preserve_stack_heap_methods {
                if let Some(constructor_method) = constructor_method {
                    self.emit_stack_heap_compatibility_methods(constructor_method, output)?;
                }
            }

            output.push_str("}\n");
        }

        for interface_name in &trait_targets {
            let methods = trait_methods.remove(*interface_name).unwrap_or_default();
            output.push_str("\nimpl");
            output.push_str(&declaration.generics_definition);
            output.push(' ');
            output.push_str(*interface_name);
            output.push_str(" for ");
            output.push_str(&declaration.name);
            output.push_str(&declaration.generics_usage);
            output.push_str("\n{\n");

            for method in methods {
                emit_attributes_into(&method.attributes, output, "    ");
                let normalized_method = self.normalize_trait_impl_method(method)?;
                emit_indented_source_into(normalized_method.trim(), output, "    ");
                output.push_str("\n\n");
            }

            output.push_str("}\n");
        }

        return Ok(());
    }


    fn emit_stack_heap_compatibility_methods(&self, constructor: &MethodDeclaration, output: &mut String) -> Result<()> {
        let parameter_source = extract_signature_parameters(&constructor.source)?;
        let parameters = extract_method_parameters(&constructor.source)?;
        let argument_names = parameters
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        output.push_str("    pub fn new_boxed(");
        output.push_str(&parameter_source);
        output.push_str(") -> Box<Self>\n");
        output.push_str("    {\n");
        output.push_str("        return Box::new(Self::new(");
        output.push_str(&argument_names);
        output.push_str("));\n");
        output.push_str("    }\n\n");

        output.push_str("    #[allow(non_snake_case)]\n");
        output.push_str("    pub fn Stack(");
        output.push_str(&parameter_source);
        output.push_str(") -> Self\n");
        output.push_str("    {\n");
        output.push_str("        return Self::new(");
        output.push_str(&argument_names);
        output.push_str(");\n");
        output.push_str("    }\n\n");

        output.push_str("    #[allow(non_snake_case)]\n");
        output.push_str("    pub fn Heap(");
        output.push_str(&parameter_source);
        output.push_str(") -> Box<Self>\n");
        output.push_str("    {\n");
        output.push_str("        return Self::new_boxed(");
        output.push_str(&argument_names);
        output.push_str(");\n");
        output.push_str("    }\n\n");

        return Ok(());
    }

    fn normalize_field(&self, field: &FieldDeclaration) -> Result<String> {
        let trimmed = field.source.trim().trim_end_matches(';').trim();

        if trimmed.is_empty() {
            return Err(anyhow!("empty field declaration"));
        }

        let normalized = if let Some(rest) = trimmed.strip_prefix("private ") {
            rest.trim().to_string()
        } else if let Some(rest) = trimmed.strip_prefix("public ") {
            format!("pub {}", rest.trim())
        } else {
            trimmed.to_string()
        };

        if normalized.starts_with("pub private ") || normalized.starts_with("private pub ") {
            return Err(anyhow!("invalid field visibility: {}", field.source));
        }

        return Ok(self.apply_statement_level_features(&rewrite_this_keyword(&normalized, self.flags.this_receiver))?);
    }

    fn normalize_inherent_method(&self, method: &MethodDeclaration) -> Result<String> {
        let trimmed = method.source.trim_start();
        let without_private = strip_leading_word(trimmed, "private");

        let with_pub = if let Some(rest) = strip_leading_word(without_private, "public").strip_prefix("fn ") {
            format!("pub fn {}", rest)
        } else {
            without_private.to_string()
        };

        return self.apply_statement_level_features(&rewrite_this_keyword(&with_pub, self.flags.this_receiver));
    }

    fn normalize_trait_impl_method(&self, method: &MethodDeclaration) -> Result<String> {
        let trimmed = method.source.trim_start();
        let without_visibility = strip_any_leading_visibility(trimmed);
        return self.apply_statement_level_features(&rewrite_this_keyword(without_visibility, self.flags.this_receiver));
    }

    /// Apply features that rewrite statements or expressions inside otherwise valid Rust code.
    ///
    /// This is deliberately separate from class/interface lowering. A reader can
    /// understand it as: first emit Rust-shaped items, then clean up small Rust Plus
    /// expression sugar such as `Account::Heap(...)`.
    fn apply_statement_level_features(&self, source: &str) -> Result<String> {
        let after_legacy_csharp_declarations = rewrite_csharp_variable_declarations(source, self.context, self.flags)?;
        let after_new_expressions = rewrite_new_expressions(&after_legacy_csharp_declarations, self.context, self.flags)?;
        let after_stack_heap_initializers = rewrite_stack_heap_initializers(&after_new_expressions, self.context, self.flags)?;

        return Ok(after_stack_heap_initializers);
    }
}

fn emit_indented_source_into(source: &str, output: &mut String, indentation: &str) {
    for line in source.lines() {
        if line.trim().is_empty() {
            output.push('\n');
            continue;
        }

        output.push_str(indentation);
        output.push_str(line.trim_end());
        output.push('\n');
    }
}

fn emit_attributes_into(attributes: &[String], output: &mut String, indentation: &str) {
    for attribute in attributes {
        for line in attribute.lines() {
            output.push_str(indentation);
            output.push_str(line.trim_end());
            output.push('\n');
        }
    }
}

fn extract_signature_parameters(method_source: &str) -> Result<String> {
    let fn_index = method_source
        .find("fn")
        .ok_or_else(|| anyhow!("expected fn in method source"))?;
    let name_start = method_source[fn_index + 2..]
        .find(|character: char| !character.is_ascii_whitespace())
        .map(|offset| fn_index + 2 + offset)
        .ok_or_else(|| anyhow!("missing method name"))?;
    let open_parenthesis = method_source[name_start..]
        .find('(')
        .map(|offset| name_start + offset)
        .ok_or_else(|| anyhow!("missing '(' in method signature"))?;
    let close_parenthesis = find_matching_parenthesis_in_method(method_source, open_parenthesis)?;

    return Ok(method_source[open_parenthesis + 1..close_parenthesis].trim().to_string());
}

fn find_matching_parenthesis_in_method(source: &str, open_parenthesis: usize) -> Result<usize> {
    let mut depth = 0usize;
    let mut index = open_parenthesis;
    let mut in_string = false;
    let mut in_char = false;

    while index < source.len() {
        let character = source.as_bytes()[index] as char;

        if in_string {
            if character == '\\' {
                index += 2;
                continue;
            }

            if character == '"' {
                in_string = false;
            }

            index += 1;
            continue;
        }

        if in_char {
            if character == '\\' {
                index += 2;
                continue;
            }

            if character == '\'' {
                in_char = false;
            }

            index += 1;
            continue;
        }

        if character == '"' {
            in_string = true;
            index += 1;
            continue;
        }

        if character == '\'' {
            in_char = true;
            index += 1;
            continue;
        }

        if character == '(' {
            depth += 1;
        } else if character == ')' {
            depth = depth.saturating_sub(1);

            if depth == 0 {
                return Ok(index);
            }
        }

        index += 1;
    }

    return Err(anyhow!("unclosed '(' in method signature"));
}

fn strip_any_leading_visibility(value: &str) -> &str {
    let without_pub = strip_leading_word(value, "pub");
    let without_public = strip_leading_word(without_pub, "public");
    let without_private = strip_leading_word(without_public, "private");
    return without_private;
}

fn strip_leading_word<'a>(value: &'a str, word: &str) -> &'a str {
    let trimmed = value.trim_start();

    if let Some(rest) = trimmed.strip_prefix(word) {
        if rest.chars().next().is_some_and(|character| character.is_ascii_whitespace()) {
            return rest.trim_start();
        }
    }

    return trimmed;
}

fn to_snake_case(value: &str) -> String {
    let mut output = String::new();

    for (index, character) in value.chars().enumerate() {
        if character == ':' || character == '<' {
            break;
        }

        if character.is_ascii_uppercase() {
            if index > 0 && !output.ends_with('_') {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            continue;
        }

        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            continue;
        }

        if !output.ends_with('_') {
            output.push('_');
        }
    }

    if output.is_empty() {
        return String::from("base");
    }

    return output.trim_matches('_').to_string();
}
