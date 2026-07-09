use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

enum OutputPart {
    Literal(String),
    Value(TokenStream),
}

#[proc_macro]
pub fn output(input: TokenStream) -> TokenStream {
    let body = generate_output_body("__output_buffer", input);

    let mut inner = parse_generated_tokens(
        r#"
            let __stdout_handle = ::std::io::stdout();
            let mut __stdout_lock = __stdout_handle.lock();
            let mut __output_buffer: ::std::vec::Vec<u8> = ::std::vec::Vec::with_capacity(2048);
        "#,
    );

    inner.extend(body);
    inner.extend(parse_generated_tokens(
        r#"
            let _ = ::std::io::Write::write_all(&mut __stdout_lock, &__output_buffer);
        "#,
    ));

    block(inner)
}

#[proc_macro]
pub fn output_to(input: TokenStream) -> TokenStream {
    let (writer, output_tokens) = parse_writer_input(input);
    let body = generate_output_body("__writer", output_tokens);

    let mut inner = parse_generated_tokens("let mut __writer = ");
    inner.extend(writer);
    inner.extend(parse_generated_tokens(";"));
    inner.extend(body);
    block(inner)
}

#[proc_macro]
pub fn output_buffered_to(input: TokenStream) -> TokenStream {
    let (writer, output_tokens) = parse_writer_input(input);
    let body = generate_output_body("__output_buffer", output_tokens);

    let mut inner = parse_generated_tokens("let mut __writer = ");
    inner.extend(writer);
    inner.extend(parse_generated_tokens(
        r#";
            let mut __output_buffer: ::std::vec::Vec<u8> = ::std::vec::Vec::with_capacity(2048);
        "#,
    ));
    inner.extend(body);
    inner.extend(parse_generated_tokens(
        "let _ = ::std::io::Write::write_all(&mut __writer, &__output_buffer);",
    ));
    block(inner)
}

fn parse_writer_input(input: TokenStream) -> (TokenStream, TokenStream) {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut cursor = 0;

    if !matches!(tokens.get(cursor), Some(TokenTree::Ident(ident)) if ident.to_string() == "writer")
    {
        return (
            compile_error_expression("expected `writer: ...,`"),
            TokenStream::new(),
        );
    }

    cursor += 1;

    if !matches!(tokens.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ':') {
        return (
            compile_error_expression("expected `writer: ...,`"),
            TokenStream::new(),
        );
    }

    cursor += 1;

    let mut writer_tokens = Vec::new();

    while cursor < tokens.len() {
        if matches!(tokens.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
            cursor += 1;
            break;
        }

        writer_tokens.push(tokens[cursor].clone());
        cursor += 1;
    }

    let writer = TokenStream::from_iter(writer_tokens);
    let rest = TokenStream::from_iter(tokens[cursor..].iter().cloned());

    (writer, rest)
}

fn generate_output_body(writer_name: &str, input: TokenStream) -> TokenStream {
    let lines = parse_lines(input);
    let mut code = TokenStream::new();

    for line in lines {
        let mut line_code = parse_generated_tokens(
            r#"
                let mut __previous_written = None;
                let mut __pending_space = false;
                let mut __value_was_just_written = false;
            "#,
        );

        for part in parse_parts(line) {
            match part {
                OutputPart::Literal(literal) => {
                    if literal.trim().is_empty() {
                        continue;
                    }

                    line_code.extend(parse_generated_tokens(&format!(
                        "let _ = ::io_macros_project::write_template_literal_segment(&mut {writer_name}, {}, &mut __previous_written, &mut __pending_space, &mut __value_was_just_written);\n",
                        rust_string_literal(&literal),
                    )));
                }
                OutputPart::Value(value) => {
                    line_code.extend(parse_generated_tokens(&format!(
                        "if __previous_written.is_some_and(|__previous| ::io_macros_project::should_write_pending_template_space(__previous, b'x')) {{
                            let _ = ::std::io::Write::write_all(&mut {writer_name}, b\" \");
                        }}
                        __pending_space = false;"
                    )));
                    line_code.extend(value_write_block(value, writer_name));
                    line_code.extend(parse_generated_tokens(
                        "__previous_written = Some(b'x'); __value_was_just_written = true;",
                    ));
                }
            }
        }

        line_code.extend(parse_generated_tokens(&format!(
            "let _ = ::io_macros_project::write_template_newline(&mut {writer_name});\n"
        )));
        code.extend(block(line_code));
    }

    code
}

fn parse_lines(input: TokenStream) -> Vec<Vec<TokenTree>> {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut lines = Vec::new();
    let mut current = Vec::new();
    let mut cursor = 0;
    let mut saw_line_start = false;

    while cursor < tokens.len() {
        if is_line_start(&tokens, cursor) {
            if saw_line_start && !current.is_empty() {
                lines.push(current);
                current = Vec::new();
            }

            saw_line_start = true;
            cursor += 2;
            continue;
        }

        if saw_line_start {
            current.push(tokens[cursor].clone());
        }

        cursor += 1;
    }

    if saw_line_start && !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn parse_parts(tokens: Vec<TokenTree>) -> Vec<OutputPart> {
    let mut parts = Vec::new();
    let mut literal_tokens = Vec::new();

    for token in tokens {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                flush_literal(&mut parts, &mut literal_tokens);
                parts.push(OutputPart::Value(group.stream()));
            }
            TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                flush_literal(&mut parts, &mut literal_tokens);
                parts.push(OutputPart::Literal("[".to_owned()));
                parts.extend(parse_parts(group.stream().into_iter().collect()));
                parts.push(OutputPart::Literal("]".to_owned()));
            }
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                literal_tokens.push(group_to_literal(group, "(", ")"));
            }
            TokenTree::Group(group) => {
                literal_tokens.push(group_to_literal(group, "", ""));
            }
            token => {
                literal_tokens.push(token);
            }
        }
    }

    flush_literal(&mut parts, &mut literal_tokens);
    parts
}

fn flush_literal(parts: &mut Vec<OutputPart>, literal_tokens: &mut Vec<TokenTree>) {
    if literal_tokens.is_empty() {
        return;
    }

    let literal = tokens_to_string(std::mem::take(literal_tokens));

    if !literal.is_empty() {
        parts.push(OutputPart::Literal(literal));
    }
}

fn group_to_literal(group: Group, open: &str, close: &str) -> TokenTree {
    let literal = format!("{open}{}{close}", group.stream());
    let group = Group::new(Delimiter::None, literal.parse().unwrap_or_default());
    TokenTree::Group(group)
}

fn is_line_start(tokens: &[TokenTree], cursor: usize) -> bool {
    matches!(tokens.get(cursor), Some(TokenTree::Punct(first)) if first.as_char() == '<')
        && matches!(tokens.get(cursor + 1), Some(TokenTree::Punct(second)) if second.as_char() == '<')
}

fn tokens_to_string(tokens: Vec<TokenTree>) -> String {
    TokenStream::from_iter(tokens).to_string()
}

fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

fn parse_generated_tokens(code: &str) -> TokenStream {
    code.parse().expect("generated output macro should parse")
}

fn compile_error_expression(message: &str) -> TokenStream {
    parse_generated_tokens(&format!("{{ compile_error!({}); }}", rust_string_literal(message)))
}

fn block(inner: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Group(Group::new(Delimiter::Brace, inner)))
}

fn value_write_block(value: TokenStream, writer_name: &str) -> TokenStream {
    let mut body = parse_generated_tokens("use ::io_macros_project::OutputValue as _;");
    body.extend(parse_generated_tokens("let _ = "));

    let mut inner_value = parse_generated_tokens("&");
    inner_value.extend(TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        value,
    ))));

    body.extend(TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        inner_value,
    ))));
    body.extend(parse_generated_tokens(&format!(
        ".write_output_value(&mut {writer_name});"
    )));

    block(body)
}
