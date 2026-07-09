use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

enum OutputPart {
    Literal(String),
    Value(TokenStream),
}

enum OutputOperation {
    Literal(String),
    Value(TokenStream),
}

struct TemplateState {
    previous_written: Option<u8>,
    pending_space: bool,
    value_was_just_written: bool,
}

impl TemplateState {
    fn new() -> Self {
        Self {
            previous_written: None,
            pending_space: false,
            value_was_just_written: false,
        }
    }
}

#[proc_macro]
pub fn output(input: TokenStream) -> TokenStream {
    let (buffer_size, output_tokens) = parse_buffer_input(input);
    let body = generate_output_body("__output_buffer", output_tokens);

    let mut inner = parse_generated_tokens(
        r#"
            let __stdout_handle = ::std::io::stdout();
            let mut __stdout_lock = __stdout_handle.lock();
            let mut __output_buffer: ::std::vec::Vec<u8> = ::std::vec::Vec::with_capacity
        "#,
    );
    inner.extend(parenthesized(buffer_size));
    inner.extend(parse_generated_tokens(";"));

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
    let (buffer_size, output_tokens) = parse_buffer_input(output_tokens);
    let body = generate_output_body("__output_buffer", output_tokens);

    let mut inner = parse_generated_tokens("let mut __writer = ");
    inner.extend(writer);
    inner.extend(parse_generated_tokens(
        r#";
            let mut __output_buffer: ::std::vec::Vec<u8> = ::std::vec::Vec::with_capacity
        "#,
    ));
    inner.extend(parenthesized(buffer_size));
    inner.extend(parse_generated_tokens(";"));
    inner.extend(body);
    inner.extend(parse_generated_tokens(
        "let _ = ::std::io::Write::write_all(&mut __writer, &__output_buffer);",
    ));
    block(inner)
}

#[proc_macro]
pub fn output_reusing_to(input: TokenStream) -> TokenStream {
    let (writer, output_tokens) = parse_writer_input(input);
    let (buffer, output_tokens) = parse_reusable_buffer_input(output_tokens);
    let body = generate_output_body("__output_buffer", output_tokens);

    let mut inner = parse_generated_tokens("let mut __writer = ");
    inner.extend(writer);
    inner.extend(parse_generated_tokens("; let mut __output_buffer = "));
    inner.extend(buffer);
    inner.extend(parse_generated_tokens("; __output_buffer.clear();"));
    inner.extend(body);
    inner.extend(parse_generated_tokens(
        "let _ = ::std::io::Write::write_all(&mut __writer, __output_buffer.as_slice());",
    ));
    block(inner)
}

fn parse_buffer_input(input: TokenStream) -> (TokenStream, TokenStream) {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut cursor = 0;

    if !matches!(tokens.get(cursor), Some(TokenTree::Ident(ident)) if ident.to_string() == "buffer")
    {
        return (parse_generated_tokens("2048"), TokenStream::from_iter(tokens));
    }

    cursor += 1;

    if !matches!(tokens.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ':') {
        return (
            compile_error_expression("expected `buffer: 8 KB,`"),
            TokenStream::new(),
        );
    }

    cursor += 1;

    let mut buffer_tokens = Vec::new();

    while cursor < tokens.len() {
        if matches!(tokens.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
            cursor += 1;
            break;
        }

        buffer_tokens.push(tokens[cursor].clone());
        cursor += 1;
    }

    if buffer_tokens.is_empty() {
        return (
            compile_error_expression("expected `buffer: 8 KB,`"),
            TokenStream::new(),
        );
    }

    let buffer_size = parse_buffer_size(buffer_tokens);
    let rest = TokenStream::from_iter(tokens[cursor..].iter().cloned());

    (buffer_size, rest)
}

fn parse_reusable_buffer_input(input: TokenStream) -> (TokenStream, TokenStream) {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut cursor = 0;

    if !matches!(tokens.get(cursor), Some(TokenTree::Ident(ident)) if ident.to_string() == "buffer")
    {
        return (
            compile_error_expression("expected `buffer: &mut buffer,`"),
            TokenStream::new(),
        );
    }

    cursor += 1;

    if !matches!(tokens.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ':') {
        return (
            compile_error_expression("expected `buffer: &mut buffer,`"),
            TokenStream::new(),
        );
    }

    cursor += 1;

    let mut buffer_tokens = Vec::new();

    while cursor < tokens.len() {
        if matches!(tokens.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
            cursor += 1;
            break;
        }

        buffer_tokens.push(tokens[cursor].clone());
        cursor += 1;
    }

    if buffer_tokens.is_empty() {
        return (
            compile_error_expression("expected `buffer: &mut buffer,`"),
            TokenStream::new(),
        );
    }

    let buffer = TokenStream::from_iter(buffer_tokens);
    let rest = TokenStream::from_iter(tokens[cursor..].iter().cloned());

    (buffer, rest)
}

fn parse_buffer_size(tokens: Vec<TokenTree>) -> TokenStream {
    if tokens.len() != 2 {
        return compile_error_expression("expected `buffer: 8 KB,`");
    }

    let Some(amount) = parse_integer_literal(&tokens[0]) else {
        return compile_error_expression("expected a whole-number buffer size, for example `buffer: 8 KB,`");
    };

    let Some(multiplier) = parse_buffer_unit_multiplier(&tokens[1]) else {
        return compile_error_expression("expected buffer unit `BYTES`, `KB`, or `MB`");
    };

    let Some(bytes) = amount.checked_mul(multiplier) else {
        return compile_error_expression("buffer size is too large");
    };

    parse_generated_tokens(&bytes.to_string())
}

fn parse_integer_literal(token: &TokenTree) -> Option<usize> {
    let TokenTree::Literal(literal) = token else {
        return None;
    };

    let literal_text = literal.to_string();
    let mut digits = String::new();

    for character in literal_text.chars() {
        if character == '_' {
            continue;
        }

        if !character.is_ascii_digit() {
            return None;
        }

        digits.push(character);
    }

    if digits.is_empty() {
        return None;
    }

    digits.parse().ok()
}

fn parse_buffer_unit_multiplier(token: &TokenTree) -> Option<usize> {
    let TokenTree::Ident(ident) = token else {
        return None;
    };

    let unit = ident.to_string().to_ascii_lowercase();

    match unit.as_str() {
        "b" | "byte" | "bytes" => Some(1),
        "kb" | "kib" | "kilobyte" | "kilobytes" => Some(1024),
        "mb" | "mib" | "megabyte" | "megabytes" => Some(1024 * 1024),
        _ => None,
    }
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
        let mut line_code = TokenStream::new();

        for operation in render_line_operations(parse_parts(line)) {
            match operation {
                OutputOperation::Literal(literal) => {
                    line_code.extend(literal_write_statement(&literal, writer_name));
                }
                OutputOperation::Value(value) => {
                    line_code.extend(value_write_block(value, writer_name));
                }
            }
        }

        code.extend(block(line_code));
    }

    code
}

fn render_line_operations(parts: Vec<OutputPart>) -> Vec<OutputOperation> {
    let mut operations = Vec::new();
    let mut state = TemplateState::new();

    for part in parts {
        match part {
            OutputPart::Literal(literal) => {
                let rendered_literal = render_literal_segment(&literal, &mut state);
                push_literal_operation(&mut operations, rendered_literal);
            }
            OutputPart::Value(value) => {
                let prefix = render_value_prefix(&mut state);
                push_literal_operation(&mut operations, prefix);
                operations.push(OutputOperation::Value(value));
                state.previous_written = Some(b'x');
                state.value_was_just_written = true;
            }
        }
    }

    push_literal_operation(&mut operations, "\n".to_owned());

    operations
}

fn push_literal_operation(operations: &mut Vec<OutputOperation>, literal: String) {
    if literal.is_empty() {
        return;
    }

    if let Some(OutputOperation::Literal(previous_literal)) = operations.last_mut() {
        previous_literal.push_str(&literal);
        return;
    }

    operations.push(OutputOperation::Literal(literal));
}

fn render_literal_segment(segment: &str, state: &mut TemplateState) -> String {
    let bytes = segment.as_bytes();
    let mut cursor = 0;
    let mut rendered = String::new();

    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_whitespace() {
            state.pending_space = true;
            cursor += 1;
            continue;
        }

        let needs_space_after_value_or_bracket =
            (state.value_was_just_written || state.previous_written.is_some_and(|previous| previous == b']'))
                && is_identifier_start(bytes[cursor]);
        let needs_space_before_bracket = bytes[cursor] == b'['
            && state
                .previous_written
                .is_some_and(|previous| previous.is_ascii_alphanumeric() || previous == b']');
        let needs_pending_template_space = state.pending_space
            && state
                .previous_written
                .is_some_and(|previous| should_write_pending_template_space(previous, bytes[cursor]));

        if needs_space_after_value_or_bracket
            || needs_space_before_bracket
            || needs_pending_template_space
        {
            rendered.push(' ');
            state.previous_written = Some(b' ');
        }

        state.pending_space = false;
        state.value_was_just_written = false;

        let run_start = cursor;

        while cursor < bytes.len() && !bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        rendered.push_str(&segment[run_start..cursor]);
        state.previous_written = Some(bytes[cursor - 1]);
    }

    rendered
}

fn render_value_prefix(state: &mut TemplateState) -> String {
    let mut rendered = String::new();

    if state
        .previous_written
        .is_some_and(|previous| should_write_pending_template_space(previous, b'x'))
    {
        rendered.push(' ');
    }

    state.pending_space = false;

    rendered
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

fn literal_write_statement(literal: &str, writer_name: &str) -> TokenStream {
    parse_generated_tokens(&format!(
        "let _ = ::std::io::Write::write_all(&mut {writer_name}, {}.as_bytes());",
        rust_string_literal(literal),
    ))
}

fn block(inner: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Group(Group::new(Delimiter::Brace, inner)))
}

fn parenthesized(inner: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)))
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

fn should_write_pending_template_space(previous: u8, next: u8) -> bool {
    !matches!(previous, b'[' | b'(' | b'{' | b'/' | b'^')
        && !matches!(
            next,
            b',' | b'.' | b':' | b';' | b'!' | b'?' | b']' | b')' | b'}' | b'/' | b'^'
        )
}

fn is_identifier_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic()
}
