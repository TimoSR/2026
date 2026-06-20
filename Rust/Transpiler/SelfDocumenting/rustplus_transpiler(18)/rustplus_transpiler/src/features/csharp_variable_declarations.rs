//! Legacy C#-style declaration support.
//!
//! This feature is disabled by default. It remains in the codebase for backward
//! compatibility, but the standard Rust Plus syntax should use Rust-shaped `let`
//! bindings plus `Type::Stack(...)` / `Type::Heap(...)`.
//!
//! New language design should not be added here unless the project deliberately
//! re-enables C#-style surface syntax.

use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::scanner::{has_identifier_boundary_after, has_identifier_boundary_before, ScannerState};
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct CSharpVariableDeclarationsFeature;

impl LanguageFeature for CSharpVariableDeclarationsFeature {
    fn name(&self) -> &'static str {
        return "csharp_variable_declarations";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.csharp_variable_declarations;
    }

    fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            match item {
                TopLevelItem::Raw(source) => {
                    if contains_csharp_variable_declaration(source, context) {
                        bail!("language feature 'csharp_variable_declarations' is disabled, but C#-style variable declarations were used");
                    }
                }
                TopLevelItem::Interface(declaration)
                | TopLevelItem::AbstractClass(declaration)
                | TopLevelItem::Class(declaration) => {
                    if contains_csharp_variable_declaration(&declaration.body, context) {
                        bail!(
                            "language feature 'csharp_variable_declarations' is disabled, but '{}' uses C#-style variable declarations",
                            declaration.name
                        );
                    }
                }
            }
        }

        return Ok(());
    }
}

pub fn rewrite_csharp_variable_declarations(
    source: &str,
    context: &SemanticContext,
    flags: &FeatureFlags,
) -> Result<String> {
    if !flags.csharp_variable_declarations {
        return Ok(source.to_string());
    }

    let mut output = String::with_capacity(source.len() + source.len() / 8);
    let mut line_start = 0usize;
    let mut scanner_state = ScannerState::Normal;

    while line_start < source.len() {
        let line_end = source[line_start..]
            .find('\n')
            .map(|offset| line_start + offset + 1)
            .unwrap_or(source.len());
        let line = &source[line_start..line_end];

        if matches!(scanner_state, ScannerState::Normal) {
            output.push_str(&rewrite_line_if_csharp_declaration(line, context, flags)?);
        } else {
            output.push_str(line);
        }

        update_multiline_comment_state(line, &mut scanner_state);
        line_start = line_end;
    }

    return Ok(output);
}

fn contains_csharp_variable_declaration(source: &str, context: &SemanticContext) -> bool {
    for line in source.lines() {
        if try_parse_csharp_variable_declaration_line(line, context).is_some() {
            return true;
        }
    }

    return false;
}

fn rewrite_line_if_csharp_declaration(
    line: &str,
    context: &SemanticContext,
    flags: &FeatureFlags,
) -> Result<String> {
    let Some(parsed) = try_parse_csharp_variable_declaration_line(line, context) else {
        return Ok(line.to_string());
    };

    if parsed.uses_new_expression && !flags.new_expressions {
        bail!("language feature 'new_expressions' is disabled, but C#-style 'new Type(...)' was used");
    }

    if parsed.requires_trait_object && !flags.interface_object_sugar {
        bail!("language feature 'interface_object_sugar' is disabled, but interface object syntax was used");
    }

    let mut output = String::with_capacity(line.len() + 64);
    output.push_str(parsed.indentation);

    // C# local variables are mutable by default. Rust-native `let` syntax remains unchanged.
    output.push_str("let mut ");
    output.push_str(parsed.variable_name);
    output.push_str(": ");

    if parsed.requires_trait_object {
        output.push_str("Box<dyn ");
        output.push_str(parsed.declared_type);
        output.push_str("> = Box::new(");
        output.push_str(&rewrite_initializer(parsed.initializer, context, flags)?);
        output.push(')');
    } else {
        output.push_str(parsed.declared_type);
        output.push_str(" = ");
        output.push_str(&rewrite_initializer(parsed.initializer, context, flags)?);
    }

    output.push(';');
    output.push_str(parsed.trailing_comment);

    if parsed.ends_with_newline {
        output.push('\n');
    }

    return Ok(output);
}

fn rewrite_initializer(initializer: &str, context: &SemanticContext, flags: &FeatureFlags) -> Result<String> {
    let Some(new_expression) = parse_new_expression(initializer.trim()) else {
        return Ok(initializer.trim().to_string());
    };

    if !flags.new_expressions {
        bail!("language feature 'new_expressions' is disabled, but C#-style 'new Type(...)' was used");
    }

    return Ok(rewrite_new_expression_to_constructor_call(&new_expression, context));
}

#[derive(Debug, Clone, Copy)]
struct ParsedCSharpDeclaration<'a> {
    indentation: &'a str,
    declared_type: &'a str,
    variable_name: &'a str,
    initializer: &'a str,
    trailing_comment: &'a str,
    ends_with_newline: bool,
    requires_trait_object: bool,
    uses_new_expression: bool,
}

fn try_parse_csharp_variable_declaration_line<'a>(
    line: &'a str,
    context: &SemanticContext,
) -> Option<ParsedCSharpDeclaration<'a>> {
    let ends_with_newline = line.ends_with('\n');
    let line_without_newline = line.trim_end_matches(&['\r', '\n'][..]);
    let indentation_len = line_without_newline.len() - line_without_newline.trim_start().len();
    let indentation = &line_without_newline[..indentation_len];
    let after_indent = &line_without_newline[indentation_len..];

    if after_indent.is_empty()
        || after_indent.starts_with("//")
        || after_indent.starts_with("/*")
        || after_indent.starts_with('*')
        || after_indent.starts_with("let ")
        || after_indent.starts_with("const ")
        || after_indent.starts_with("static ")
        || after_indent.starts_with("return ")
        || after_indent.starts_with("fn ")
        || after_indent.starts_with("pub ")
        || after_indent.starts_with("private ")
        || after_indent.starts_with("public ")
        || after_indent.starts_with("class ")
        || after_indent.starts_with("interface ")
        || after_indent.starts_with("abstract ")
        || after_indent.starts_with("use ")
        || after_indent.starts_with("mod ")
    {
        return None;
    }

    let (code, trailing_comment) = split_trailing_line_comment(after_indent);
    let code = code.trim_end();

    if !code.ends_with(';') {
        return None;
    }

    let statement = code.trim_end_matches(';').trim();
    let mut cursor = 0usize;
    let explicit_mut = if statement.starts_with("mut ") {
        cursor = 4;
        true
    } else {
        false
    };

    cursor = skip_ascii_whitespace(statement, cursor);
    let type_start = cursor;
    let type_end = read_type_token_end(statement, type_start)?;
    let declared_type = statement[type_start..type_end].trim();

    if declared_type.is_empty() || declared_type.contains('=') || declared_type == "let" || declared_type == "mut" {
        return None;
    }

    cursor = skip_ascii_whitespace(statement, type_end);
    let variable_start = cursor;
    let variable_end = read_identifier_end(statement, variable_start);

    if variable_end == variable_start {
        return None;
    }

    let variable_name = &statement[variable_start..variable_end];
    cursor = skip_ascii_whitespace(statement, variable_end);

    if !statement[cursor..].starts_with('=') {
        return None;
    }

    cursor += 1;
    let initializer = statement[cursor..].trim();

    if initializer.is_empty() {
        return None;
    }

    let new_expression = parse_new_expression(initializer);
    let constructed_type = new_expression.as_ref().map(|value| value.type_name.as_str());
    let requires_trait_object = constructed_type.is_some_and(|constructed_type| {
        let declared_base_name = type_name_without_generics(declared_type);
        let constructed_base_name = type_name_without_generics(constructed_type);
        return declared_base_name != constructed_base_name && context.is_known_or_likely_interface(declared_base_name);
    });

    // Avoid rewriting ordinary Rust expressions like `Result value = ...` unless this is clearly the
    // C#-style object-construction form this feature owns.
    if new_expression.is_none() && !explicit_mut {
        return None;
    }

    return Some(ParsedCSharpDeclaration {
        indentation,
        declared_type,
        variable_name,
        initializer,
        trailing_comment,
        ends_with_newline,
        requires_trait_object,
        uses_new_expression: new_expression.is_some(),
    });
}

#[derive(Debug, Clone)]
struct ParsedNewExpression {
    type_name: String,
    arguments: String,
}

fn parse_new_expression(value: &str) -> Option<ParsedNewExpression> {
    let trimmed = value.trim();

    if !trimmed.starts_with("new") || !has_identifier_boundary_after(trimmed, "new".len()) {
        return None;
    }

    let mut cursor = skip_ascii_whitespace(trimmed, "new".len());
    let type_start = cursor;
    let type_end = read_type_token_end(trimmed, type_start)?;
    let type_name = trimmed[type_start..type_end].trim();

    if type_name.is_empty() {
        return None;
    }

    cursor = skip_ascii_whitespace(trimmed, type_end);

    if !trimmed[cursor..].starts_with('(') {
        return None;
    }

    let close_paren = find_matching_parenthesis(trimmed, cursor)?;

    if !trimmed[close_paren + 1..].trim().is_empty() {
        return None;
    }

    return Some(ParsedNewExpression {
        type_name: type_name.to_string(),
        arguments: trimmed[cursor + 1..close_paren].to_string(),
    });
}

pub fn rewrite_new_expressions(source: &str, context: &SemanticContext, flags: &FeatureFlags) -> Result<String> {
    if !flags.new_expressions || !source.contains("new ") {
        return Ok(source.to_string());
    }

    let mut output = String::with_capacity(source.len());
    let mut index = 0usize;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        match state {
            ScannerState::Normal => {
                if source[index..].starts_with("//") {
                    output.push_str("//");
                    index += 2;
                    state = ScannerState::LineComment;
                    continue;
                }

                if source[index..].starts_with("/*") {
                    output.push_str("/*");
                    index += 2;
                    state = ScannerState::BlockComment;
                    continue;
                }

                if source[index..].starts_with("new")
                    && has_identifier_boundary_before(source, index)
                    && has_identifier_boundary_after(source, index + "new".len())
                    && previous_non_whitespace_char(source, index).map_or(true, |character| character != ':' && character != '.')
                {
                    if let Some(rewritten) = try_rewrite_new_expression_at(source, index, context) {
                        output.push_str(&rewritten.rust);
                        index = rewritten.next_index;
                        continue;
                    }
                }

                let character = source.as_bytes()[index] as char;
                output.push(character);
                index += 1;

                if character == '"' {
                    state = ScannerState::String;
                } else if character == '\'' {
                    state = ScannerState::Char;
                }
            }
            ScannerState::LineComment => {
                let character = source.as_bytes()[index] as char;
                output.push(character);
                index += 1;

                if character == '\n' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::BlockComment => {
                if source[index..].starts_with("*/") {
                    output.push_str("*/");
                    index += 2;
                    state = ScannerState::Normal;
                    continue;
                }

                output.push(source.as_bytes()[index] as char);
                index += 1;
            }
            ScannerState::String => {
                let character = source.as_bytes()[index] as char;
                output.push(character);
                index += 1;

                if character == '\\' && index < source.len() {
                    output.push(source.as_bytes()[index] as char);
                    index += 1;
                    continue;
                }

                if character == '"' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::Char => {
                let character = source.as_bytes()[index] as char;
                output.push(character);
                index += 1;

                if character == '\\' && index < source.len() {
                    output.push(source.as_bytes()[index] as char);
                    index += 1;
                    continue;
                }

                if character == '\'' {
                    state = ScannerState::Normal;
                }
            }
        }
    }

    return Ok(output);
}

#[derive(Debug, Clone)]
struct RewrittenNewExpression {
    rust: String,
    next_index: usize,
}

fn try_rewrite_new_expression_at(source: &str, start: usize, context: &SemanticContext) -> Option<RewrittenNewExpression> {
    let cursor = skip_ascii_whitespace(source, start + "new".len());
    let type_start = cursor;
    let type_end = read_type_token_end(source, type_start)?;
    let type_name = source[type_start..type_end].trim();
    let paren_start = skip_ascii_whitespace(source, type_end);

    if !source[paren_start..].starts_with('(') {
        return None;
    }

    let paren_end = find_matching_parenthesis(source, paren_start)?;
    let arguments = &source[paren_start + 1..paren_end];

    return Some(RewrittenNewExpression {
        rust: rewrite_new_expression_to_constructor_call(
            &ParsedNewExpression {
                type_name: type_name.to_string(),
                arguments: arguments.to_string(),
            },
            context,
        ),
        next_index: paren_end + 1,
    });
}

fn rewrite_new_expression_to_constructor_call(new_expression: &ParsedNewExpression, context: &SemanticContext) -> String {
    let arguments = rewrite_constructor_arguments(&new_expression.type_name, &new_expression.arguments, context);
    return format!("{}::new({})", new_expression.type_name, arguments);
}

fn rewrite_constructor_arguments(type_name: &str, arguments: &str, context: &SemanticContext) -> String {
    let Some(parameters) = context.constructor_parameters(type_name) else {
        return arguments.to_string();
    };

    let parsed_arguments = split_constructor_arguments(arguments);

    if parsed_arguments.len() != parameters.len() {
        return arguments.to_string();
    }

    return parsed_arguments
        .iter()
        .zip(parameters.iter())
        .map(|(argument, parameter)| {
            let argument = argument.trim();

            if is_owned_string_type(&parameter.type_name) && is_plain_string_literal(argument) {
                return format!("{}.to_string()", argument);
            }

            return argument.to_string();
        })
        .collect::<Vec<String>>()
        .join(", ");
}

fn is_owned_string_type(type_name: &str) -> bool {
    let type_name = type_name.trim();
    return type_name == "String" || type_name == "std::string::String" || type_name == "alloc::string::String";
}

fn is_plain_string_literal(value: &str) -> bool {
    let value = value.trim();

    if !value.starts_with('"') || !value.ends_with('"') || value.len() < 2 {
        return false;
    }

    let mut escaped = false;

    for character in value[1..value.len() - 1].chars() {
        if escaped {
            escaped = false;
            continue;
        }

        if character == '\\' {
            escaped = true;
        }
    }

    return !escaped;
}

fn split_constructor_arguments(arguments: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut index = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut generic_depth = 0usize;
    let mut state = ScannerState::Normal;

    while index < arguments.len() {
        match state {
            ScannerState::Normal => {
                if arguments[index..].starts_with("//") {
                    state = ScannerState::LineComment;
                    index += 2;
                    continue;
                }

                if arguments[index..].starts_with("/*") {
                    state = ScannerState::BlockComment;
                    index += 2;
                    continue;
                }

                let character = arguments.as_bytes()[index] as char;

                match character {
                    '(' => paren_depth += 1,
                    ')' => paren_depth = paren_depth.saturating_sub(1),
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth = bracket_depth.saturating_sub(1),
                    '{' => brace_depth += 1,
                    '}' => brace_depth = brace_depth.saturating_sub(1),
                    '<' => generic_depth += 1,
                    '>' => generic_depth = generic_depth.saturating_sub(1),
                    ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 && generic_depth == 0 => {
                        parts.push(arguments[start..index].trim().to_string());
                        start = index + 1;
                    }
                    '"' => state = ScannerState::String,
                    '\'' => state = ScannerState::Char,
                    _ => {}
                }

                index += 1;
            }
            ScannerState::LineComment => {
                if arguments.as_bytes()[index] as char == '\n' {
                    state = ScannerState::Normal;
                }
                index += 1;
            }
            ScannerState::BlockComment => {
                if arguments[index..].starts_with("*/") {
                    state = ScannerState::Normal;
                    index += 2;
                    continue;
                }
                index += 1;
            }
            ScannerState::String => {
                let character = arguments.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < arguments.len() {
                    index += 1;
                    continue;
                }

                if character == '"' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::Char => {
                let character = arguments.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < arguments.len() {
                    index += 1;
                    continue;
                }

                if character == '\'' {
                    state = ScannerState::Normal;
                }
            }
        }
    }

    let tail = arguments[start..].trim();

    if !tail.is_empty() || !arguments.trim().is_empty() {
        parts.push(tail.to_string());
    }

    return parts;
}

fn split_trailing_line_comment(value: &str) -> (&str, &str) {
    let mut index = 0usize;
    let mut state = ScannerState::Normal;

    while index < value.len() {
        match state {
            ScannerState::Normal => {
                if value[index..].starts_with("//") {
                    return (&value[..index], &value[index..]);
                }

                let character = value.as_bytes()[index] as char;

                if character == '"' {
                    state = ScannerState::String;
                } else if character == '\'' {
                    state = ScannerState::Char;
                }

                index += 1;
            }
            ScannerState::String => {
                let character = value.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < value.len() {
                    index += 1;
                    continue;
                }

                if character == '"' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::Char => {
                let character = value.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < value.len() {
                    index += 1;
                    continue;
                }

                if character == '\'' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::LineComment | ScannerState::BlockComment => {
                index += 1;
            }
        }
    }

    return (value, "");
}

fn update_multiline_comment_state(line: &str, state: &mut ScannerState) {
    let mut index = 0usize;

    while index < line.len() {
        match *state {
            ScannerState::Normal => {
                if line[index..].starts_with("//") {
                    return;
                }

                if line[index..].starts_with("/*") {
                    *state = ScannerState::BlockComment;
                    index += 2;
                    continue;
                }

                let character = line.as_bytes()[index] as char;
                if character == '"' {
                    *state = ScannerState::String;
                } else if character == '\'' {
                    *state = ScannerState::Char;
                }

                index += 1;
            }
            ScannerState::BlockComment => {
                if line[index..].starts_with("*/") {
                    *state = ScannerState::Normal;
                    index += 2;
                    continue;
                }

                index += 1;
            }
            ScannerState::String => {
                let character = line.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < line.len() {
                    index += 1;
                    continue;
                }

                if character == '"' {
                    *state = ScannerState::Normal;
                }
            }
            ScannerState::Char => {
                let character = line.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < line.len() {
                    index += 1;
                    continue;
                }

                if character == '\'' {
                    *state = ScannerState::Normal;
                }
            }
            ScannerState::LineComment => return,
        }
    }
}

fn find_matching_parenthesis(source: &str, open_parenthesis: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut index = open_parenthesis;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        match state {
            ScannerState::Normal => {
                if source[index..].starts_with("//") {
                    state = ScannerState::LineComment;
                    index += 2;
                    continue;
                }

                if source[index..].starts_with("/*") {
                    state = ScannerState::BlockComment;
                    index += 2;
                    continue;
                }

                let character = source.as_bytes()[index] as char;

                if character == '(' {
                    depth += 1;
                } else if character == ')' {
                    depth = depth.saturating_sub(1);

                    if depth == 0 {
                        return Some(index);
                    }
                } else if character == '"' {
                    state = ScannerState::String;
                } else if character == '\'' {
                    state = ScannerState::Char;
                }

                index += 1;
            }
            ScannerState::LineComment => {
                if source.as_bytes()[index] as char == '\n' {
                    state = ScannerState::Normal;
                }
                index += 1;
            }
            ScannerState::BlockComment => {
                if source[index..].starts_with("*/") {
                    state = ScannerState::Normal;
                    index += 2;
                    continue;
                }
                index += 1;
            }
            ScannerState::String => {
                let character = source.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < source.len() {
                    index += 1;
                    continue;
                }

                if character == '"' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::Char => {
                let character = source.as_bytes()[index] as char;
                index += 1;

                if character == '\\' && index < source.len() {
                    index += 1;
                    continue;
                }

                if character == '\'' {
                    state = ScannerState::Normal;
                }
            }
        }
    }

    return None;
}

fn read_type_token_end(source: &str, mut index: usize) -> Option<usize> {
    let mut generic_depth = 0usize;
    let mut saw_character = false;

    while index < source.len() {
        let character = source.as_bytes()[index] as char;

        match character {
            '<' => {
                generic_depth += 1;
                saw_character = true;
                index += 1;
            }
            '>' => {
                generic_depth = generic_depth.saturating_sub(1);
                saw_character = true;
                index += 1;
            }
            ':' => {
                if source[index..].starts_with("::") {
                    saw_character = true;
                    index += 2;
                } else {
                    break;
                }
            }
            character if character.is_ascii_alphanumeric() || character == '_' => {
                saw_character = true;
                index += 1;
            }
            character if character.is_ascii_whitespace() && generic_depth == 0 => break,
            ',' | ')' | '(' | '=' | ';' if generic_depth == 0 => break,
            _ if generic_depth > 0 => index += 1,
            _ => break,
        }
    }

    if !saw_character {
        return None;
    }

    return Some(index);
}

fn read_identifier_end(source: &str, mut index: usize) -> usize {
    while index < source.len() {
        let character = source.as_bytes()[index] as char;

        if character.is_ascii_alphanumeric() || character == '_' {
            index += 1;
            continue;
        }

        break;
    }

    return index;
}

fn skip_ascii_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() && source.as_bytes()[index].is_ascii_whitespace() {
        index += 1;
    }

    return index;
}

fn type_name_without_generics(value: &str) -> &str {
    return value.split('<').next().unwrap_or(value).trim();
}

fn previous_non_whitespace_char(source: &str, index: usize) -> Option<char> {
    if index == 0 {
        return None;
    }

    return source[..index].chars().rev().find(|character| !character.is_whitespace());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ParameterDeclaration;
    use std::collections::{HashMap, HashSet};

    fn csharp_flags() -> FeatureFlags {
        let mut flags = FeatureFlags::default();
        flags.csharp_variable_declarations = true;
        flags.new_expressions = true;
        flags.interface_object_sugar = true;
        return flags;
    }

    fn context_with_iaccount() -> SemanticContext {
        let mut interface_methods = HashMap::new();
        interface_methods.insert(String::from("IAccount"), HashSet::from([String::from("balance")]));

        return SemanticContext {
            interface_methods,
            class_names: HashSet::from([String::from("Account")]),
            class_bodies: HashMap::new(),
            declaration_kinds: HashMap::new(),
            constructor_parameters: HashMap::from([(
                String::from("Account"),
                vec![ParameterDeclaration {
                    name: String::from("id"),
                    type_name: String::from("String"),
                }],
            )]),
            default_initializers: HashSet::new(),
        };
    }

    #[test]
    fn rewrites_interface_declaration_to_boxed_dyn_trait_object() {
        let flags = csharp_flags();
        let context = context_with_iaccount();
        let source = "    IAccount account = new Account();\n";
        let rewritten = rewrite_csharp_variable_declarations(source, &context, &flags).unwrap();

        assert_eq!(rewritten, "    let mut account: Box<dyn IAccount> = Box::new(Account::new());\n");
    }

    #[test]
    fn rewrites_concrete_declaration_to_typed_let_binding() {
        let flags = csharp_flags();
        let context = context_with_iaccount();
        let source = "    Account account = new Account();\n";
        let rewritten = rewrite_csharp_variable_declarations(source, &context, &flags).unwrap();

        assert_eq!(rewritten, "    let mut account: Account = Account::new();\n");
    }


    #[test]
    fn rewrites_string_literal_arguments_for_string_constructor_parameters() {
        let flags = csharp_flags();
        let context = context_with_iaccount();
        let source = "    IAccount account = new Account(\"account-1\");\n";
        let rewritten = rewrite_csharp_variable_declarations(source, &context, &flags).unwrap();

        assert_eq!(
            rewritten,
            "    let mut account: Box<dyn IAccount> = Box::new(Account::new(\"account-1\".to_string()));\n"
        );
    }

    #[test]
    fn leaves_strings_and_comments_unchanged() {
        let flags = csharp_flags();
        let context = context_with_iaccount();
        let source = "    // IAccount account = new Account();\n    println!(\"new Account()\");\n";
        let rewritten = rewrite_csharp_variable_declarations(source, &context, &flags).unwrap();

        assert_eq!(rewritten, source);
    }
}
