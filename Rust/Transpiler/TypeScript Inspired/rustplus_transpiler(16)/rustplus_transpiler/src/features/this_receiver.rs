use crate::ast::TopLevelItem;
use crate::config::FeatureFlags;
use crate::features::LanguageFeature;
use crate::scanner::{has_identifier_boundary_after, has_identifier_boundary_before, ScannerState};
use crate::transpiler::SemanticContext;
use anyhow::{bail, Result};

pub struct ThisReceiverFeature;

impl LanguageFeature for ThisReceiverFeature {
    fn name(&self) -> &'static str {
        return "this_receiver";
    }

    fn enabled(&self, flags: &FeatureFlags) -> bool {
        return flags.this_receiver;
    }

    fn validate(&self, items: &[TopLevelItem], _context: &SemanticContext, flags: &FeatureFlags) -> Result<()> {
        if self.enabled(flags) {
            return Ok(());
        }

        for item in items {
            match item {
                TopLevelItem::Interface(declaration) | TopLevelItem::AbstractClass(declaration) | TopLevelItem::Class(declaration) => {
                    if contains_this_keyword(&declaration.body) {
                        bail!("language feature 'this_receiver' is disabled, but '{}' uses 'this'", declaration.name);
                    }
                }
                TopLevelItem::Raw(_) => {}
            }
        }

        return Ok(());
    }
}

pub fn rewrite_this_keyword(source: &str, enabled: bool) -> String {
    if !enabled || !source.contains("this") {
        return source.to_string();
    }

    let mut output = String::with_capacity(source.len());
    let mut index = 0usize;
    let bytes = source.as_bytes();
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

                if source[index..].starts_with("this")
                    && has_identifier_boundary_before(source, index)
                    && has_identifier_boundary_after(source, index + "this".len())
                {
                    output.push_str("self");
                    index += "this".len();
                    continue;
                }

                let character = bytes[index] as char;
                output.push(character);
                index += 1;

                if character == '"' {
                    state = ScannerState::String;
                    continue;
                }

                if character == '\'' {
                    state = ScannerState::Char;
                    continue;
                }
            }
            ScannerState::LineComment => {
                let character = bytes[index] as char;
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

                output.push(bytes[index] as char);
                index += 1;
            }
            ScannerState::String => {
                let character = bytes[index] as char;
                output.push(character);
                index += 1;

                if character == '\\' && index < source.len() {
                    output.push(bytes[index] as char);
                    index += 1;
                    continue;
                }

                if character == '"' {
                    state = ScannerState::Normal;
                }
            }
            ScannerState::Char => {
                let character = bytes[index] as char;
                output.push(character);
                index += 1;

                if character == '\\' && index < source.len() {
                    output.push(bytes[index] as char);
                    index += 1;
                    continue;
                }

                if character == '\'' {
                    state = ScannerState::Normal;
                }
            }
        }
    }

    return output;
}

fn contains_this_keyword(source: &str) -> bool {
    return rewrite_this_keyword(source, true) != source;
}
