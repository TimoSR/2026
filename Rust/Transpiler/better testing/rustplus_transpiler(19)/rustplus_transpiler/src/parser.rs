use crate::ast::{ClassBody, DeclarationKind, FieldDeclaration, MethodDeclaration, ParameterDeclaration, TopLevelItem, TypeDeclaration};
use crate::scanner::{
    find_matching_brace, find_next_char_or_semicolon_outside_strings, find_next_char_outside_strings,
    has_identifier_boundary_after, has_identifier_boundary_before, next_non_ws_after_word, read_identifier_end,
    skip_whitespace, split_once_top_level, split_top_level_commas, starts_with_word, update_scanner_state, ScannerState,
};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FoundDeclaration {
    start: usize,
    kind: DeclarationKind,
}

pub fn parse_top_level_items(source: &str) -> Result<Vec<TopLevelItem>> {
    let mut items = Vec::new();
    let mut cursor = 0;

    while cursor < source.len() {
        let next = find_next_declaration(source, cursor);

        let Some(found) = next else {
            if cursor < source.len() {
                items.push(TopLevelItem::Raw(source[cursor..].to_string()));
            }
            break;
        };

        let prefix = &source[cursor..found.start];
        let (raw_prefix, attributes) = split_trailing_attribute_sequence(prefix)?;

        if !raw_prefix.is_empty() {
            items.push(TopLevelItem::Raw(raw_prefix));
        }

        let (declaration, next_cursor) = parse_type_declaration(source, found.start, found.kind, attributes)?;
        let item = match found.kind {
            DeclarationKind::Interface => TopLevelItem::Interface(declaration),
            DeclarationKind::AbstractClass => TopLevelItem::AbstractClass(declaration),
            DeclarationKind::Class => TopLevelItem::Class(declaration),
        };

        items.push(item);
        cursor = next_cursor;
    }

    return Ok(items);
}

pub fn parse_class_body(body: &str) -> Result<ClassBody> {
    let mut fields = Vec::new();
    let mut methods = Vec::new();
    let mut cursor = 0;

    while cursor < body.len() {
        cursor = skip_whitespace(body, cursor);

        if cursor >= body.len() {
            break;
        }

        let (attributes, after_attributes) = parse_leading_attributes(body, cursor)?;
        cursor = skip_whitespace(body, after_attributes);

        if cursor >= body.len() {
            if !attributes.is_empty() {
                return Err(anyhow!("attributes must be followed by a field or method declaration"));
            }
            break;
        }

        if is_method_start(body, cursor) {
            let (method, next_cursor) = parse_method(body, cursor, attributes)?;
            methods.push(method);
            cursor = next_cursor;
            continue;
        }

        let semicolon = find_next_char_outside_strings(body, cursor, ';')
            .ok_or_else(|| anyhow!("expected field ending ';' in class body near byte {}", cursor))?;
        let field = body[cursor..=semicolon].trim().to_string();

        if !field.is_empty() || !attributes.is_empty() {
            fields.push(FieldDeclaration { attributes, source: field });
        }

        cursor = semicolon + 1;
    }

    return Ok(ClassBody { fields, methods });
}

pub fn extract_methods(body: &str) -> Result<Vec<MethodDeclaration>> {
    let mut methods = Vec::new();
    let mut cursor = 0;

    while cursor < body.len() {
        cursor = skip_until_method(body, cursor);

        if cursor >= body.len() {
            break;
        }

        let (method, next_cursor) = parse_method(body, cursor, Vec::new())?;
        methods.push(method);
        cursor = next_cursor;
    }

    return Ok(methods);
}

/// Extracts parameter names and types from a parsed method declaration.
///
/// This intentionally stays lightweight. It supports the Rust Plus constructor/signature
/// forms the transpiler owns, including generic parameter types like `Option<String>`.
pub fn extract_method_parameters(method_source: &str) -> Result<Vec<ParameterDeclaration>> {
    let fn_index = method_source
        .find("fn")
        .ok_or_else(|| anyhow!("expected fn in method source"))?;
    let name_start = skip_whitespace(method_source, fn_index + 2);
    let name_end = read_identifier_end(method_source, name_start);

    if name_end == name_start {
        return Err(anyhow!("missing method name"));
    }

    let open_parenthesis = find_next_char_outside_strings(method_source, name_end, '(')
        .ok_or_else(|| anyhow!("missing '(' in method signature"))?;
    let close_parenthesis = find_matching_parenthesis(method_source, open_parenthesis)?;
    let parameters_source = method_source[open_parenthesis + 1..close_parenthesis].trim();

    if parameters_source.is_empty() {
        return Ok(Vec::new());
    }

    let mut parameters = Vec::new();

    for parameter_source in split_top_level_commas(parameters_source) {
        let parameter_source = parameter_source.trim();

        if parameter_source == "self"
            || parameter_source == "&self"
            || parameter_source == "&mut self"
            || parameter_source == "this"
            || parameter_source == "&this"
            || parameter_source == "&mut this"
            || parameter_source == "mut self"
            || parameter_source == "mut this"
        {
            continue;
        }

        let Some((name, type_name)) = split_once_top_level(parameter_source, ':') else {
            continue;
        };

        let name = name.trim().strip_prefix("mut ").unwrap_or(name.trim()).trim();
        let type_name = type_name.trim();

        if !name.is_empty() && !type_name.is_empty() {
            parameters.push(ParameterDeclaration {
                name: name.to_string(),
                type_name: type_name.to_string(),
            });
        }
    }

    return Ok(parameters);
}

pub fn contains_attribute_syntax(source: &str) -> bool {
    let mut index = 0usize;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) && is_attribute_start(source, index) {
            return true;
        }

        index += 1;
    }

    return false;
}

fn find_matching_parenthesis(source: &str, open_parenthesis: usize) -> Result<usize> {
    let mut depth = 0usize;
    let mut index = open_parenthesis;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) {
            let character = source.as_bytes()[index] as char;

            if character == '(' {
                depth += 1;
            }

            if character == ')' {
                depth = depth.saturating_sub(1);

                if depth == 0 {
                    return Ok(index);
                }
            }
        }

        index += 1;
    }

    return Err(anyhow!("unclosed '(' at byte {}", open_parenthesis));
}

fn find_next_declaration(source: &str, start: usize) -> Option<FoundDeclaration> {
    let candidates = [
        ("pub abstract class", DeclarationKind::AbstractClass),
        ("abstract class", DeclarationKind::AbstractClass),
        ("pub interface", DeclarationKind::Interface),
        ("interface", DeclarationKind::Interface),
        ("pub class", DeclarationKind::Class),
        ("class", DeclarationKind::Class),
    ];

    let mut index = start;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if !matches!(state, ScannerState::Normal) {
            index += 1;
            continue;
        }

        for (keyword, kind) in candidates {
            if source[index..].starts_with(keyword)
                && has_identifier_boundary_before(source, index)
                && has_identifier_boundary_after(source, index + keyword.len())
            {
                return Some(FoundDeclaration { start: index, kind });
            }
        }

        index += 1;
    }

    return None;
}

fn parse_type_declaration(
    source: &str,
    start: usize,
    kind: DeclarationKind,
    attributes: Vec<String>,
) -> Result<(TypeDeclaration, usize)> {
    let open_brace = find_next_char_outside_strings(source, start, '{')
        .ok_or_else(|| anyhow!("missing '{{' for declaration starting at byte {}", start))?;
    let close_brace = find_matching_brace(source, open_brace)?;

    let header = source[start..open_brace].trim();
    let visibility = if header.starts_with("pub ") { Some(String::from("pub")) } else { None };
    let body = source[open_brace + 1..close_brace].to_string();
    let header_without_keyword = strip_declaration_keyword(header, kind)?;
    let parsed_header = parse_type_header(header_without_keyword)?;

    return Ok((
        TypeDeclaration {
            attributes,
            visibility,
            name: parsed_header.name,
            generics_definition: parsed_header.generics_definition,
            generics_usage: parsed_header.generics_usage,
            bases: parsed_header.bases,
            body,
        },
        close_brace + 1,
    ));
}

fn strip_declaration_keyword(header: &str, kind: DeclarationKind) -> Result<&str> {
    let rest = if let Some(rest) = header.strip_prefix("pub ") {
        rest.trim_start()
    } else {
        header
    };

    let expected = match kind {
        DeclarationKind::Interface => "interface",
        DeclarationKind::AbstractClass => "abstract class",
        DeclarationKind::Class => "class",
    };

    let stripped = rest
        .strip_prefix(expected)
        .ok_or_else(|| anyhow!("invalid declaration header: {}", header))?
        .trim();

    return Ok(stripped);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedTypeHeader {
    name: String,
    generics_definition: String,
    generics_usage: String,
    bases: Vec<String>,
}

fn parse_type_header(header_without_keyword: &str) -> Result<ParsedTypeHeader> {
    let (left, right) = split_once_top_level(header_without_keyword, ':')
        .map(|(left, right)| (left.trim(), Some(right.trim())))
        .unwrap_or((header_without_keyword.trim(), None));

    if left.is_empty() {
        return Err(anyhow!("missing type name"));
    }

    let (name, generics_definition, generics_usage) = parse_name_and_generics(left)?;
    let bases = right.map(split_top_level_commas).unwrap_or_default();

    return Ok(ParsedTypeHeader {
        name,
        generics_definition,
        generics_usage,
        bases,
    });
}

fn parse_name_and_generics(value: &str) -> Result<(String, String, String)> {
    let trimmed = value.trim();

    if let Some(generic_start) = trimmed.find('<') {
        let name = trimmed[..generic_start].trim().to_string();
        let generics_definition = trimmed[generic_start..].trim().to_string();
        let generics_usage = generic_usage_from_definition(&generics_definition);

        if name.is_empty() {
            return Err(anyhow!("missing type name in header: {}", value));
        }

        return Ok((name, generics_definition, generics_usage));
    }

    return Ok((trimmed.to_string(), String::new(), String::new()));
}

fn generic_usage_from_definition(generics_definition: &str) -> String {
    if generics_definition.is_empty() {
        return String::new();
    }

    let inner = generics_definition.trim().trim_start_matches('<').trim_end_matches('>');
    let names = inner
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.split(|character: char| {
                character == ':' || character == '=' || character.is_ascii_whitespace()
            })
            .next()
            .unwrap_or(part)
            .trim()
            .to_string()
        })
        .collect::<Vec<String>>();

    if names.is_empty() {
        return String::new();
    }

    return format!("<{}>", names.join(", "));
}

fn parse_method(source: &str, start: usize, attributes: Vec<String>) -> Result<(MethodDeclaration, usize)> {
    let fn_start = if starts_with_word(source, start, "pub") {
        next_non_ws_after_word(source, start, "pub").ok_or_else(|| anyhow!("invalid pub fn at byte {}", start))?
    } else if starts_with_word(source, start, "public") {
        next_non_ws_after_word(source, start, "public").ok_or_else(|| anyhow!("invalid public fn at byte {}", start))?
    } else if starts_with_word(source, start, "private") {
        next_non_ws_after_word(source, start, "private").ok_or_else(|| anyhow!("invalid private fn at byte {}", start))?
    } else {
        start
    };

    if !starts_with_word(source, fn_start, "fn") {
        return Err(anyhow!("expected fn at byte {}", start));
    }

    let name_start = skip_whitespace(source, fn_start + 2);
    let name_end = read_identifier_end(source, name_start);

    if name_end == name_start {
        return Err(anyhow!("missing method name at byte {}", name_start));
    }

    let name = source[name_start..name_end].to_string();
    let terminator = find_next_char_or_semicolon_outside_strings(source, name_end)?;

    match terminator.character {
        ';' => {
            let method_source = source[start..=terminator.index].trim().to_string();
            return Ok((MethodDeclaration { attributes, name, source: method_source }, terminator.index + 1));
        }
        '{' => {
            let close_brace = find_matching_brace(source, terminator.index)?;
            let method_source = source[start..=close_brace].trim().to_string();
            return Ok((MethodDeclaration { attributes, name, source: method_source }, close_brace + 1));
        }
        _ => return Err(anyhow!("unexpected method terminator")),
    }
}

fn skip_until_method(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() {
        if is_method_start(source, cursor) {
            return cursor;
        }

        cursor += 1;
    }

    return cursor;
}

fn is_method_start(source: &str, cursor: usize) -> bool {
    if starts_with_word(source, cursor, "fn") {
        return true;
    }

    for visibility in ["pub", "public", "private"] {
        if starts_with_word(source, cursor, visibility)
            && next_non_ws_after_word(source, cursor, visibility).is_some_and(|idx| starts_with_word(source, idx, "fn"))
        {
            return true;
        }
    }

    return false;
}

fn split_trailing_attribute_sequence(prefix: &str) -> Result<(String, Vec<String>)> {
    let mut index = 0usize;
    let mut state = ScannerState::Normal;

    while index < prefix.len() {
        update_scanner_state(prefix, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) && is_outer_attribute_start(prefix, index) {
            if let Some(attributes) = parse_attribute_sequence_exact(prefix, index)? {
                return Ok((prefix[..index].to_string(), attributes));
            }
        }

        index += 1;
    }

    return Ok((prefix.to_string(), Vec::new()));
}

fn parse_attribute_sequence_exact(source: &str, start: usize) -> Result<Option<Vec<String>>> {
    let mut attributes = Vec::new();
    let mut cursor = start;

    loop {
        cursor = skip_whitespace(source, cursor);

        if cursor >= source.len() {
            return Ok(if attributes.is_empty() { None } else { Some(attributes) });
        }

        if !is_outer_attribute_start(source, cursor) {
            return Ok(None);
        }

        let end = parse_attribute_end(source, cursor)?;
        attributes.push(source[cursor..end].trim().to_string());
        cursor = end;
    }
}

fn parse_leading_attributes(source: &str, start: usize) -> Result<(Vec<String>, usize)> {
    let mut attributes = Vec::new();
    let mut cursor = start;

    loop {
        cursor = skip_whitespace(source, cursor);

        if cursor >= source.len() || !is_outer_attribute_start(source, cursor) {
            return Ok((attributes, cursor));
        }

        let end = parse_attribute_end(source, cursor)?;
        attributes.push(source[cursor..end].trim().to_string());
        cursor = end;
    }
}

fn is_attribute_start(source: &str, index: usize) -> bool {
    return source[index..].starts_with("#[") || source[index..].starts_with("#![");
}

fn is_outer_attribute_start(source: &str, index: usize) -> bool {
    return source[index..].starts_with("#[") && !source[index..].starts_with("#![");
}

fn parse_attribute_end(source: &str, start: usize) -> Result<usize> {
    if !is_attribute_start(source, start) {
        return Err(anyhow!("expected attribute at byte {}", start));
    }

    let bracket_start = if source[start..].starts_with("#![") { start + 2 } else { start + 1 };
    let mut depth = 0usize;
    let mut index = bracket_start;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) {
            let character = source.as_bytes()[index] as char;

            if character == '[' {
                depth += 1;
            } else if character == ']' {
                depth = depth.saturating_sub(1);

                if depth == 0 {
                    return Ok(index + 1);
                }
            }
        }

        index += 1;
    }

    return Err(anyhow!("unclosed attribute starting at byte {}", start));
}
