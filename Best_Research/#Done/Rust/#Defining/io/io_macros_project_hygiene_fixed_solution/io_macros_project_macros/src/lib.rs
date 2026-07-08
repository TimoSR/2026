use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

#[proc_macro]
pub fn output(input: TokenStream) -> TokenStream {
    let lines = collect_non_empty_lines(input);

    expand_output_stdout(&lines).unwrap_or_else(compile_error)
}

#[proc_macro]
pub fn output_to(input: TokenStream) -> TokenStream {
    let mut lines = collect_non_empty_lines(input);

    if lines.is_empty() {
        return "{}".parse().expect("empty block should parse");
    }

    let first_line = lines.remove(0);

    let Some(writer_expr) = parse_writer_line(&first_line) else {
        return compile_error(
            "output_to! expects the first line to be: writer: <writer expression>",
        );
    };

    expand_output_writer(&writer_expr, &lines).unwrap_or_else(compile_error)
}

fn collect_non_empty_lines(input: TokenStream) -> Vec<Vec<TokenTree>> {
    let mut lines: Vec<Vec<TokenTree>> = Vec::new();

    for token in input {
        let line = token.span().line();

        if let Some(last_line) = lines.last_mut() {
            let last_token_line = last_line
                .last()
                .map(|token| token.span().line())
                .unwrap_or(line);

            if last_token_line == line {
                last_line.push(token);
                continue;
            }
        }

        lines.push(vec![token]);
    }

    lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect()
}

fn parse_writer_line(line: &[TokenTree]) -> Option<String> {
    let colon_index = line
        .iter()
        .position(|token| matches!(token, TokenTree::Punct(punct) if punct.as_char() == ':'))?;

    let before_colon = &line[..colon_index];

    if before_colon.len() != 1 {
        return None;
    }

    match &before_colon[0] {
        TokenTree::Ident(ident) if ident.to_string() == "writer" => {}
        _ => return None,
    }

    let mut expr_tokens = line[colon_index + 1..].to_vec();

    if matches!(expr_tokens.last(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
        expr_tokens.pop();
    }

    if expr_tokens.is_empty() {
        return None;
    }

    Some(tokens_to_rust_source(&expr_tokens))
}

fn expand_output_stdout(lines: &[Vec<TokenTree>]) -> Result<TokenStream, &'static str> {
    let mut generated = String::from(
        "{\
            use ::std::io::Write as _;\
            let __stdout_handle = ::std::io::stdout();\
            let mut __output_writer = __stdout_handle.lock();",
    );

    push_writeln_calls(&mut generated, lines)?;

    generated.push('}');

    generated
        .parse()
        .map_err(|_| "failed to generate output! expansion")
}

fn expand_output_writer(
    writer_expr: &str,
    lines: &[Vec<TokenTree>],
) -> Result<TokenStream, &'static str> {
    let mut generated = format!(
        "{{\
            use ::std::io::Write as _;\
            let mut __output_writer = {writer_expr};"
    );

    push_writeln_calls(&mut generated, lines)?;

    generated.push('}');

    generated
        .parse()
        .map_err(|_| "failed to generate output_to! expansion")
}

fn push_writeln_calls(
    generated: &mut String,
    lines: &[Vec<TokenTree>],
) -> Result<(), &'static str> {
    for line in lines {
        let OutputLine {
            format_string,
            expressions,
        } = build_output_line(line)?;

        generated.push_str("let _ = ::std::writeln!(&mut __output_writer, ");
        generated.push_str(&rust_string_literal(&format_string));

        for expression in expressions {
            generated.push_str(", ");
            generated.push_str(&expression);
        }

        generated.push_str(");");
    }

    Ok(())
}

struct OutputLine {
    format_string: String,
    expressions: Vec<String>,
}

fn build_output_line(line: &[TokenTree]) -> Result<OutputLine, &'static str> {
    let mut format_string = String::new();
    let mut expressions = Vec::new();
    let mut previous = PieceKind::Start;

    for token in line {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                append_placeholder(&mut format_string, &mut previous);

                let expression = group.stream().to_string();

                if expression.trim().is_empty() {
                    return Err("empty interpolation braces are not allowed");
                }

                expressions.push(expression);
            }
            TokenTree::Group(group) => {
                let piece = render_non_interpolation_group(group);
                append_word_like(&mut format_string, &piece, &mut previous);
            }
            TokenTree::Ident(ident) => {
                append_word_like(&mut format_string, &ident.to_string(), &mut previous);
            }
            TokenTree::Literal(literal) => {
                append_word_like(&mut format_string, &literal.to_string(), &mut previous);
            }
            TokenTree::Punct(punct) => {
                append_punctuation(&mut format_string, punct.as_char(), &mut previous);
            }
        }
    }

    Ok(OutputLine {
        format_string: format_string.trim_end().to_string(),
        expressions,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PieceKind {
    Start,
    WordLike,
    Placeholder,
    Slash,
    Opening,
    Punctuation,
    Equals,
}

fn append_placeholder(output: &mut String, previous: &mut PieceKind) {
    append_space_if_needed(output, *previous);
    output.push_str("{}");
    *previous = PieceKind::Placeholder;
}

fn append_word_like(output: &mut String, piece: &str, previous: &mut PieceKind) {
    append_space_if_needed(output, *previous);
    output.push_str(piece);
    *previous = PieceKind::WordLike;
}

fn append_punctuation(output: &mut String, character: char, previous: &mut PieceKind) {
    match character {
        '=' => {
            trim_trailing_spaces(output);

            if !output.is_empty() {
                output.push(' ');
            }

            output.push('=');
            output.push(' ');
            *previous = PieceKind::Equals;
        }
        '/' => {
            trim_trailing_spaces(output);
            output.push('/');
            *previous = PieceKind::Slash;
        }
        ',' => {
            trim_trailing_spaces(output);
            output.push(',');
            output.push(' ');
            *previous = PieceKind::Punctuation;
        }
        '.' | ':' | '!' | '?' => {
            trim_trailing_spaces(output);
            output.push(character);
            output.push(' ');
            *previous = PieceKind::Punctuation;
        }
        '(' | '[' => {
            append_space_if_needed(output, *previous);
            output.push(character);
            *previous = PieceKind::Opening;
        }
        ')' | ']' => {
            trim_trailing_spaces(output);
            output.push(character);
            *previous = PieceKind::Punctuation;
        }
        '-' => {
            trim_trailing_spaces(output);

            if !output.is_empty() {
                output.push(' ');
            }

            output.push('-');
            output.push(' ');
            *previous = PieceKind::Punctuation;
        }
        '+' | '*' => {
            trim_trailing_spaces(output);

            if !output.is_empty() {
                output.push(' ');
            }

            output.push(character);
            output.push(' ');
            *previous = PieceKind::Punctuation;
        }
        _ => {
            append_space_if_needed(output, *previous);
            output.push(character);
            *previous = PieceKind::Punctuation;
        }
    }
}

fn append_space_if_needed(output: &mut String, previous: PieceKind) {
    if output.is_empty() || output.ends_with(char::is_whitespace) {
        return;
    }

    match previous {
        PieceKind::Start | PieceKind::Slash | PieceKind::Opening | PieceKind::Equals => {}
        PieceKind::WordLike | PieceKind::Placeholder | PieceKind::Punctuation => output.push(' '),
    }
}

fn trim_trailing_spaces(output: &mut String) {
    while output.ends_with(' ') {
        output.pop();
    }
}

fn render_non_interpolation_group(group: &Group) -> String {
    let (open, close) = match group.delimiter() {
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Bracket => ('[', ']'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::None => (' ', ' '),
    };

    if group.delimiter() == Delimiter::None {
        return group.stream().to_string();
    }

    format!("{open}{}{close}", group.stream())
}

fn tokens_to_rust_source(tokens: &[TokenTree]) -> String {
    tokens
        .iter()
        .map(TokenTree::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

fn rust_string_literal(value: &str) -> String {
    let mut literal = String::from("\"");

    for character in value.chars() {
        match character {
            '\\' => literal.push_str("\\\\"),
            '"' => literal.push_str("\\\""),
            '\n' => literal.push_str("\\n"),
            '\r' => literal.push_str("\\r"),
            '\t' => literal.push_str("\\t"),
            character => literal.push(character),
        }
    }

    literal.push('"');
    literal
}

fn compile_error(message: &'static str) -> TokenStream {
    format!("::std::compile_error!({});", rust_string_literal(message))
        .parse()
        .expect("compile_error expansion should parse")
}
