use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::scanner::{has_identifier_boundary_after, has_identifier_boundary_before, ScannerState};
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct StackHeapInitializersFeature;

impl LanguageFeature for StackHeapInitializersFeature {
    fn name(&self) -> &'static str {
        return "stack_heap_initializers";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.stack_heap_initializers;
    }

    fn validate(&self, items: &[TopLevelItem], context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if !self.enabled(flags) {
            for item in items {
                validate_feature_not_used_in_item(item, context)?;
            }

            return Ok(());
        }

        for item in items {
            validate_item(item, context)?;
        }

        return Ok(());
    }
}

pub fn rewrite_stack_heap_initializers(source: &str, context: &SemanticContext, flags: &FeatureFlags) -> Result<String> {
    if !flags.stack_heap_initializers || (!source.contains("::Stack") && !source.contains("::Heap")) {
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
            output.push_str(&rewrite_line_if_stack_heap_let_binding(line, context, flags)?);
        } else {
            output.push_str(line);
        }

        update_multiline_comment_state(line, &mut scanner_state);
        line_start = line_end;
    }

    if !output.contains("::Stack") && !output.contains("::Heap") {
        return Ok(output);
    }

    return rewrite_remaining_stack_heap_expression_calls(&output, context);
}


fn rewrite_remaining_stack_heap_expression_calls(source: &str, context: &SemanticContext) -> Result<String> {
    let mut output = String::with_capacity(source.len() + source.len() / 8);
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

                if let Some(call) = parse_initializer_call_at(source, index) {
                    validate_call(&call, context)?;

                    match call.kind {
                        InitializerKind::Stack => output.push_str(&lower_stack_initializer(&call, context)),
                        InitializerKind::Heap => output.push_str(&lower_heap_initializer(&call, context)),
                        InitializerKind::StackLowercase | InitializerKind::HeapLowercase => unreachable!("lowercase initializers are rejected during validation"),
                    }

                    index = call.end;
                    continue;
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

fn validate_feature_not_used_in_item(item: &TopLevelItem, context: &SemanticContext) -> Result<()> {
    match item {
        TopLevelItem::Raw(source) => {
            if contains_stack_heap_initializer(source, context)? {
                bail!("language feature 'stack_heap_initializers' is disabled, but Type::Stack(...) or Type::Heap(...) was used");
            }
        }
        TopLevelItem::Interface(declaration) | TopLevelItem::AbstractClass(declaration) | TopLevelItem::Class(declaration) => {
            if contains_stack_heap_initializer(&declaration.body, context)? {
                bail!(
                    "language feature 'stack_heap_initializers' is disabled, but '{}' uses Type::Stack(...) or Type::Heap(...)",
                    declaration.name
                );
            }
        }
    }

    return Ok(());
}

fn validate_item(item: &TopLevelItem, context: &SemanticContext) -> Result<()> {
    match item {
        TopLevelItem::Raw(source) => validate_source(source, context),
        TopLevelItem::Interface(declaration) | TopLevelItem::AbstractClass(declaration) | TopLevelItem::Class(declaration) => {
            validate_source(&declaration.body, context)
        }
    }
}

fn contains_stack_heap_initializer(source: &str, context: &SemanticContext) -> Result<bool> {
    let mut found = false;
    visit_stack_heap_calls(source, context, &mut |call| {
        found = found || matches!(call.kind, InitializerKind::Stack | InitializerKind::Heap);
        return Ok(());
    })?;

    return Ok(found);
}

fn validate_source(source: &str, context: &SemanticContext) -> Result<()> {
    visit_stack_heap_calls(source, context, &mut |call| {
        validate_call(&call, context)?;
        return Ok(());
    })?;

    return Ok(());
}

fn validate_call(call: &InitializerCall, context: &SemanticContext) -> Result<()> {
    match call.kind {
        InitializerKind::StackLowercase => {
            bail!(
                "Unknown initializer {}::stack(...).\n\nHint:\nUse {}::Stack(...) or {}::Heap(...).",
                call.type_path,
                call.type_path,
                call.type_path
            );
        }
        InitializerKind::HeapLowercase => {
            bail!(
                "Unknown initializer {}::heap(...).\n\nHint:\nUse {}::Stack(...) or {}::Heap(...).",
                call.type_path,
                call.type_path,
                call.type_path
            );
        }
        InitializerKind::Stack | InitializerKind::Heap => {}
    }

    if !context.is_constructible_type(&call.type_path) {
        bail!(
            "Cannot use {}::{}(...) because {} is not a known constructible type.",
            call.type_path,
            call.kind.source_name(),
            call.type_path
        );
    }

    if call.arguments.trim().is_empty() {
        if !context.has_default_initializer(&call.type_path) {
            bail!(
                "Cannot use {}::{}() because {} does not define a default initializer.\n\nHint:\nUse {}::{}(args) with constructor arguments, or implement Default for {}.",
                call.type_path,
                call.kind.source_name(),
                call.type_path,
                call.type_path,
                call.kind.source_name(),
                call.type_path
            );
        }

        return Ok(());
    }

    let Some(parameters) = context.constructor_parameters(&call.type_path) else {
        bail!(
            "Cannot use {}::{}(...) because {} does not define a matching new(...) constructor.",
            call.type_path,
            call.kind.source_name(),
            call.type_path
        );
    };

    let arguments = split_constructor_arguments(&call.arguments);

    if arguments.len() != parameters.len() {
        bail!(
            "Cannot use {}::{}(...) because {} does not define a matching new(...) constructor.",
            call.type_path,
            call.kind.source_name(),
            call.type_path
        );
    }

    return Ok(());
}

fn rewrite_line_if_stack_heap_let_binding(line: &str, context: &SemanticContext, flags: &FeatureFlags) -> Result<String> {
    let Some(parsed) = try_parse_stack_heap_let_binding_line(line, context)? else {
        return Ok(line.to_string());
    };

    let mut output = String::with_capacity(line.len() + 64);
    output.push_str(parsed.indentation);
    output.push_str("let ");

    if parsed.mutable {
        output.push_str("mut ");
    }

    output.push_str(parsed.variable_name);
    output.push_str(": ");
    output.push_str(&lower_stack_heap_binding_type(&parsed, context, flags)?);
    output.push_str(" = ");

    match parsed.initializer.kind {
        InitializerKind::Stack => output.push_str(&lower_stack_initializer(&parsed.initializer, context)),
        InitializerKind::Heap => output.push_str(&lower_heap_initializer(&parsed.initializer, context)),
        InitializerKind::StackLowercase | InitializerKind::HeapLowercase => unreachable!("lowercase initializers are rejected during validation"),
    }

    output.push(';');
    output.push_str(parsed.trailing_comment);

    if parsed.ends_with_newline {
        output.push('\n');
    }

    return Ok(output);
}


fn lower_stack_heap_binding_type(parsed: &ParsedStackHeapLetBinding<'_>, context: &SemanticContext, flags: &FeatureFlags) -> Result<String> {
    let constructed_type = parsed.initializer.type_path.as_str();

    let Some(declared_type) = parsed.declared_type.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(match parsed.initializer.kind {
            InitializerKind::Stack => constructed_type.to_string(),
            InitializerKind::Heap => format!("Box<{}>", constructed_type),
            InitializerKind::StackLowercase | InitializerKind::HeapLowercase => unreachable!("lowercase initializers are rejected during validation"),
        });
    };

    match parsed.initializer.kind {
        InitializerKind::Stack => {
            if is_interface_annotation_for_constructed_type(declared_type, constructed_type, context) {
                if !flags.interface_object_sugar {
                    bail!("language feature 'interface_object_sugar' is disabled, but interface object syntax was used");
                }

                bail!(
                    "Cannot bind {}::Stack(...) to interface type {}.

Hint:
Use let {}: {} = {}::Heap(...);",
                    constructed_type,
                    declared_type,
                    parsed.variable_name,
                    declared_type,
                    constructed_type
                );
            }

            return Ok(declared_type.to_string());
        }
        InitializerKind::Heap => {
            if let Some(inner_type) = parse_box_type_annotation(declared_type) {
                if context.is_known_or_likely_interface(inner_type) && !inner_type.trim_start().starts_with("dyn ") {
                    if !flags.interface_object_sugar {
                        bail!("language feature 'interface_object_sugar' is disabled, but interface object syntax was used");
                    }

                    return Ok(format!("Box<dyn {}>", inner_type));
                }

                return Ok(format!("Box<{}>", inner_type));
            }

            if is_interface_annotation_for_constructed_type(declared_type, constructed_type, context) {
                if !flags.interface_object_sugar {
                    bail!("language feature 'interface_object_sugar' is disabled, but interface object syntax was used");
                }

                return Ok(format!("Box<dyn {}>", declared_type));
            }

            if same_terminal_type_name(declared_type, constructed_type) {
                return Ok(format!("Box<{}>", constructed_type));
            }

            return Ok(declared_type.to_string());
        }
        InitializerKind::StackLowercase | InitializerKind::HeapLowercase => unreachable!("lowercase initializers are rejected during validation"),
    }
}

fn is_interface_annotation_for_constructed_type(
    declared_type: &str,
    constructed_type: &str,
    context: &SemanticContext,
) -> bool {
    return !same_terminal_type_name(declared_type, constructed_type) && context.is_known_or_likely_interface(declared_type);
}

fn same_terminal_type_name(left: &str, right: &str) -> bool {
    return terminal_type_name(type_name_without_generics(left)) == terminal_type_name(type_name_without_generics(right));
}

fn type_name_without_generics(value: &str) -> &str {
    return value.split('<').next().unwrap_or(value).trim();
}

fn terminal_type_name(value: &str) -> &str {
    return value.rsplit("::").next().unwrap_or(value).trim();
}

fn parse_box_type_annotation(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    let inner = trimmed.strip_prefix("Box<")?.strip_suffix('>')?.trim();

    if let Some(rest) = inner.strip_prefix("dyn ") {
        return Some(rest.trim());
    }

    return Some(inner);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InitializerKind {
    Stack,
    Heap,
    StackLowercase,
    HeapLowercase,
}

impl InitializerKind {
    fn source_name(self) -> &'static str {
        match self {
            Self::Stack => return "Stack",
            Self::Heap => return "Heap",
            Self::StackLowercase => return "stack",
            Self::HeapLowercase => return "heap",
        }
    }
}

#[derive(Debug, Clone)]
struct InitializerCall {
    type_path: String,
    kind: InitializerKind,
    arguments: String,
    end: usize,
}

#[derive(Debug, Clone)]
struct ParsedStackHeapLetBinding<'a> {
    indentation: &'a str,
    mutable: bool,
    variable_name: &'a str,
    declared_type: Option<&'a str>,
    initializer: InitializerCall,
    trailing_comment: &'a str,
    ends_with_newline: bool,
}

fn try_parse_stack_heap_let_binding_line<'a>(
    line: &'a str,
    context: &SemanticContext,
) -> Result<Option<ParsedStackHeapLetBinding<'a>>> {
    let ends_with_newline = line.ends_with('\n');
    let line_without_newline = line.trim_end_matches(&['\r', '\n'][..]);
    let indentation_len = line_without_newline.len() - line_without_newline.trim_start().len();
    let indentation = &line_without_newline[..indentation_len];
    let after_indent = &line_without_newline[indentation_len..];

    if after_indent.is_empty() || after_indent.starts_with("//") || after_indent.starts_with("/*") {
        return Ok(None);
    }

    let (code, trailing_comment) = split_trailing_line_comment(after_indent);
    let code = code.trim_end();

    if !code.ends_with(';') {
        return Ok(None);
    }

    let statement = code.trim_end_matches(';').trim();

    if !statement.starts_with("let") || !has_identifier_boundary_after(statement, "let".len()) {
        return Ok(None);
    }

    let mut cursor = skip_ascii_whitespace(statement, "let".len());
    let mutable = if statement[cursor..].starts_with("mut") && has_identifier_boundary_after(statement, cursor + "mut".len()) {
        cursor = skip_ascii_whitespace(statement, cursor + "mut".len());
        true
    } else {
        false
    };

    let variable_start = cursor;
    let variable_end = read_identifier_end(statement, variable_start);

    if variable_end == variable_start {
        return Ok(None);
    }

    let variable_name = &statement[variable_start..variable_end];
    cursor = skip_ascii_whitespace(statement, variable_end);

    let declared_type = if statement[cursor..].starts_with(':') {
        cursor = skip_ascii_whitespace(statement, cursor + 1);
        let type_start = cursor;
        let Some(equals_index) = find_top_level_equals(statement, type_start) else {
            return Ok(None);
        };
        let type_source = statement[type_start..equals_index].trim();

        if type_source.is_empty() {
            return Ok(None);
        }

        cursor = equals_index;
        Some(type_source)
    } else {
        None
    };

    if !statement[cursor..].starts_with('=') {
        return Ok(None);
    }

    cursor = skip_ascii_whitespace(statement, cursor + 1);
    let initializer_source = statement[cursor..].trim();
    let Some(initializer) = parse_initializer_call_exact(initializer_source) else {
        return Ok(None);
    };

    validate_call(&initializer, context)?;

    return Ok(Some(ParsedStackHeapLetBinding {
        indentation,
        mutable,
        variable_name,
        declared_type,
        initializer,
        trailing_comment,
        ends_with_newline,
    }));
}


fn find_top_level_equals(source: &str, start: usize) -> Option<usize> {
    let mut index = start;
    let mut angle_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        match state {
            ScannerState::Normal => {
                let character = source.as_bytes()[index] as char;

                match character {
                    '<' => angle_depth += 1,
                    '>' => angle_depth = angle_depth.saturating_sub(1),
                    '(' => paren_depth += 1,
                    ')' => paren_depth = paren_depth.saturating_sub(1),
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth = bracket_depth.saturating_sub(1),
                    '=' if angle_depth == 0 && paren_depth == 0 && bracket_depth == 0 => return Some(index),
                    '"' => state = ScannerState::String,
                    '\'' => state = ScannerState::Char,
                    _ => {}
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
            ScannerState::LineComment | ScannerState::BlockComment => index += 1,
        }
    }

    return None;
}

fn parse_initializer_call_exact(source: &str) -> Option<InitializerCall> {
    let call = parse_initializer_call_at(source, 0)?;

    if call.end != source.len() {
        return None;
    }

    return Some(call);
}

fn visit_stack_heap_calls<F>(source: &str, _context: &SemanticContext, visitor: &mut F) -> Result<()>
where
    F: FnMut(InitializerCall) -> Result<()>,
{
    if !source.contains("::Stack") && !source.contains("::Heap") && !source.contains("::stack") && !source.contains("::heap") {
        return Ok(());
    }

    let mut index = 0usize;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        match state {
            ScannerState::Normal => {
                if source[index..].starts_with("//") {
                    index += 2;
                    state = ScannerState::LineComment;
                    continue;
                }

                if source[index..].starts_with("/*") {
                    index += 2;
                    state = ScannerState::BlockComment;
                    continue;
                }

                if let Some(call) = parse_initializer_call_at(source, index) {
                    visitor(call.clone())?;
                    index = call.end;
                    continue;
                }

                let character = source.as_bytes()[index] as char;
                index += 1;

                if character == '"' {
                    state = ScannerState::String;
                } else if character == '\'' {
                    state = ScannerState::Char;
                }
            }
            ScannerState::LineComment => {
                let character = source.as_bytes()[index] as char;
                index += 1;

                if character == '\n' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::BlockComment => {
                if source[index..].starts_with("*/") {
                    index += 2;
                    state = ScannerState::Normal;
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

    return Ok(());
}

fn parse_initializer_call_at(source: &str, start: usize) -> Option<InitializerCall> {
    if !has_identifier_boundary_before(source, start) {
        return None;
    }

    let path_end = read_type_path_end(source, start)?;
    let type_path = &source[start..path_end];

    if !is_valid_type_path(type_path) {
        return None;
    }

    let mut cursor = path_end;

    if !source[cursor..].starts_with("::") {
        return None;
    }

    cursor += 2;
    let initializer_start = cursor;
    let initializer_end = read_identifier_end(source, initializer_start);

    if initializer_end == initializer_start {
        return None;
    }

    let initializer_name = &source[initializer_start..initializer_end];
    let kind = match initializer_name {
        "Stack" => InitializerKind::Stack,
        "Heap" => InitializerKind::Heap,
        "stack" => InitializerKind::StackLowercase,
        "heap" => InitializerKind::HeapLowercase,
        _ => return None,
    };

    cursor = skip_ascii_whitespace(source, initializer_end);

    if !source[cursor..].starts_with('(') {
        return None;
    }

    let paren_end = find_matching_parenthesis(source, cursor)?;

    return Some(InitializerCall {
        type_path: type_path.to_string(),
        kind,
        arguments: source[cursor + 1..paren_end].to_string(),
        end: paren_end + 1,
    });
}

fn lower_stack_initializer(initializer: &InitializerCall, context: &SemanticContext) -> String {
    if initializer.arguments.trim().is_empty() {
        return format!("{}::default()", initializer.type_path);
    }

    let arguments = rewrite_constructor_arguments(&initializer.type_path, &initializer.arguments, context);
    return format!("{}::new({})", initializer.type_path, arguments);
}

fn lower_heap_initializer(initializer: &InitializerCall, context: &SemanticContext) -> String {
    if initializer.arguments.trim().is_empty() {
        return format!("Box::new({}::default())", initializer.type_path);
    }

    let arguments = rewrite_constructor_arguments(&initializer.type_path, &initializer.arguments, context);
    return format!("Box::new({}::new({}))", initializer.type_path, arguments);
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

fn read_type_path_end(source: &str, mut index: usize) -> Option<usize> {
    loop {
        let segment_start = index;
        let segment_end = read_identifier_end(source, segment_start);

        if segment_end == segment_start {
            return None;
        }

        index = segment_end;

        if source[index..].starts_with("::") {
            let next = index + 2;

            if next >= source.len() {
                return None;
            }

            // Stop before the initializer segment itself.
            if source[next..].starts_with("Stack")
                || source[next..].starts_with("Heap")
                || source[next..].starts_with("stack")
                || source[next..].starts_with("heap")
            {
                break;
            }

            index = next;
            continue;
        }

        break;
    }

    return Some(index);
}

fn is_valid_type_path(value: &str) -> bool {
    if value.is_empty() || value.starts_with("::") || value.ends_with("::") {
        return false;
    }

    for segment in value.split("::") {
        if !is_pascal_like_identifier(segment) {
            return false;
        }
    }

    return true;
}

fn is_pascal_like_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };

    if !first.is_ascii_uppercase() {
        return false;
    }

    return characters.all(|character| character.is_ascii_alphanumeric() || character == '_');
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ParameterDeclaration;
    use std::collections::{HashMap, HashSet};

    fn context_with_article() -> SemanticContext {
        return SemanticContext {
            interface_methods: HashMap::new(),
            class_names: HashSet::from([String::from("Article")]),
            class_bodies: HashMap::new(),
            declaration_kinds: HashMap::new(),
            constructor_parameters: HashMap::from([(
                String::from("Article"),
                vec![
                    ParameterDeclaration {
                        name: String::from("headline"),
                        type_name: String::from("&str"),
                    },
                    ParameterDeclaration {
                        name: String::from("location"),
                        type_name: String::from("&str"),
                    },
                    ParameterDeclaration {
                        name: String::from("artist"),
                        type_name: String::from("&str"),
                    },
                ],
            )]),
            default_initializers: HashSet::from([String::from("Article")]),
        };
    }

    #[test]
    fn rewrites_stack_initializer_with_arguments() {
        let context = context_with_article();
        let flags = FeatureFlags::default();
        let source = "    let article = Article::Stack(\"My own new!\", \"Lol!\", \"stuff\");\n";
        let rewritten = rewrite_stack_heap_initializers(source, &context, &flags).unwrap();

        assert_eq!(
            rewritten,
            "    let article: Article = Article::new(\"My own new!\", \"Lol!\", \"stuff\");\n"
        );
    }

    #[test]
    fn rewrites_heap_initializer_with_arguments() {
        let context = context_with_article();
        let flags = FeatureFlags::default();
        let source = "    let boxed_article = Article::Heap(\"My own new!\", \"Lol!\", \"stuff\");\n";
        let rewritten = rewrite_stack_heap_initializers(source, &context, &flags).unwrap();

        assert_eq!(
            rewritten,
            "    let boxed_article: Box<Article> = Box::new(Article::new(\"My own new!\", \"Lol!\", \"stuff\"));\n"
        );
    }

    #[test]
    fn rewrites_typed_interface_heap_initializer() {
        let mut context = context_with_article();
        context.class_names.insert(String::from("Account"));
        context.constructor_parameters.insert(
            String::from("Account"),
            vec![ParameterDeclaration {
                name: String::from("id"),
                type_name: String::from("String"),
            }],
        );
        let flags = FeatureFlags::default();
        let source = "    let account: IAccount = Account::Heap(\"account-1\");\n";
        let rewritten = rewrite_stack_heap_initializers(source, &context, &flags).unwrap();

        assert_eq!(
            rewritten,
            "    let account: Box<dyn IAccount> = Box::new(Account::new(\"account-1\".to_string()));\n"
        );
    }

    #[test]
    fn rewrites_default_stack_initializer() {
        let context = context_with_article();
        let flags = FeatureFlags::default();
        let source = "    let article = Article::Stack();\n";
        let rewritten = rewrite_stack_heap_initializers(source, &context, &flags).unwrap();

        assert_eq!(rewritten, "    let article: Article = Article::default();\n");
    }

    #[test]
    fn rewrites_qualified_heap_initializer() {
        let context = context_with_article();
        let flags = FeatureFlags::default();
        let source = "    let boxed_article = Domain::Article::Heap(\"My own new!\", \"Lol!\", \"stuff\");\n";
        let rewritten = rewrite_stack_heap_initializers(source, &context, &flags).unwrap();

        assert_eq!(
            rewritten,
            "    let boxed_article: Box<Domain::Article> = Box::new(Domain::Article::new(\"My own new!\", \"Lol!\", \"stuff\"));\n"
        );
    }
}
