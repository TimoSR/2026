use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannerState {
    Normal,
    LineComment,
    BlockComment,
    String,
    Char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundChar {
    pub index: usize,
    pub character: char,
}

pub fn update_scanner_state(source: &str, index: &mut usize, state: &mut ScannerState) {
    let bytes = source.as_bytes();

    match *state {
        ScannerState::Normal => {
            if source[*index..].starts_with("//") {
                *state = ScannerState::LineComment;
                *index += 1;
                return;
            }

            if source[*index..].starts_with("/*") {
                *state = ScannerState::BlockComment;
                *index += 1;
                return;
            }

            match bytes[*index] as char {
                '"' => *state = ScannerState::String,
                '\'' => *state = ScannerState::Char,
                _ => {}
            }
        }
        ScannerState::LineComment => {
            if bytes[*index] as char == '\n' {
                *state = ScannerState::Normal;
            }
        }
        ScannerState::BlockComment => {
            if source[*index..].starts_with("*/") {
                *state = ScannerState::Normal;
                *index += 1;
            }
        }
        ScannerState::String => {
            if bytes[*index] as char == '\\' {
                *index += 1;
                return;
            }

            if bytes[*index] as char == '"' {
                *state = ScannerState::Normal;
            }
        }
        ScannerState::Char => {
            if bytes[*index] as char == '\\' {
                *index += 1;
                return;
            }

            if bytes[*index] as char == '\'' {
                *state = ScannerState::Normal;
            }
        }
    }
}

pub fn find_next_char_outside_strings(source: &str, start: usize, target: char) -> Option<usize> {
    let mut index = start;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) && source.as_bytes()[index] as char == target {
            return Some(index);
        }

        index += 1;
    }

    return None;
}

pub fn find_next_char_or_semicolon_outside_strings(source: &str, start: usize) -> Result<FoundChar> {
    let mut index = start;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) {
            let character = source.as_bytes()[index] as char;

            if character == '{' || character == ';' {
                return Ok(FoundChar { index, character });
            }
        }

        index += 1;
    }

    return Err(anyhow!("expected method body '{{' or declaration ';'"));
}

pub fn find_matching_brace(source: &str, open_brace: usize) -> Result<usize> {
    let mut depth = 0usize;
    let mut index = open_brace;
    let mut state = ScannerState::Normal;

    while index < source.len() {
        update_scanner_state(source, &mut index, &mut state);

        if matches!(state, ScannerState::Normal) {
            let character = source.as_bytes()[index] as char;

            if character == '{' {
                depth += 1;
            }

            if character == '}' {
                depth = depth.saturating_sub(1);

                if depth == 0 {
                    return Ok(index);
                }
            }
        }

        index += 1;
    }

    return Err(anyhow!("unclosed '{{' at byte {}", open_brace));
}

pub fn has_identifier_boundary_before(source: &str, index: usize) -> bool {
    if index == 0 {
        return true;
    }

    let character = source.as_bytes()[index - 1] as char;
    return !(character.is_ascii_alphanumeric() || character == '_');
}

pub fn has_identifier_boundary_after(source: &str, index: usize) -> bool {
    if index >= source.len() {
        return true;
    }

    let character = source.as_bytes()[index] as char;
    return !(character.is_ascii_alphanumeric() || character == '_');
}

pub fn starts_with_word(source: &str, index: usize, word: &str) -> bool {
    return source[index..].starts_with(word)
        && has_identifier_boundary_before(source, index)
        && has_identifier_boundary_after(source, index + word.len());
}

pub fn skip_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() && source.as_bytes()[index].is_ascii_whitespace() {
        index += 1;
    }

    return index;
}

pub fn next_non_ws_after_word(source: &str, index: usize, word: &str) -> Option<usize> {
    if !starts_with_word(source, index, word) {
        return None;
    }

    return Some(skip_whitespace(source, index + word.len()));
}

pub fn read_identifier_end(source: &str, mut index: usize) -> usize {
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

pub fn split_top_level_commas(value: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut generic_depth = 0usize;
    let mut start = 0usize;

    for (index, character) in value.char_indices() {
        match character {
            '<' => generic_depth += 1,
            '>' => generic_depth = generic_depth.saturating_sub(1),
            ',' if generic_depth == 0 => {
                let part = value[start..index].trim();
                if !part.is_empty() {
                    parts.push(part.to_string());
                }
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }

    let tail = value[start..].trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }

    return parts;
}

pub fn split_once_top_level(value: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut generic_depth = 0usize;

    for (index, character) in value.char_indices() {
        match character {
            '<' => generic_depth += 1,
            '>' => generic_depth = generic_depth.saturating_sub(1),
            _ if character == delimiter && generic_depth == 0 => {
                let left = &value[..index];
                let right = &value[index + character.len_utf8()..];
                return Some((left, right));
            }
            _ => {}
        }
    }

    return None;
}
