use std::fmt::{self, Display};
use std::io::{self, BufRead, Write};
use std::str::FromStr;

const INPUT_BUFFER_CAPACITY: usize = 64;
const MAX_RETAINED_INPUT_BUFFER_CAPACITY: usize = 8 * 1024;

#[derive(Debug)]
pub enum InputError {
    PromptWrite {
        label: &'static str,
        source: io::Error,
    },
    PromptFlush {
        label: &'static str,
        source: io::Error,
    },
    Read {
        label: &'static str,
        source: io::Error,
    },
    Eof {
        label: &'static str,
    },
    InvalidInputWrite {
        label: &'static str,
        source: io::Error,
    },
}

impl fmt::Display for InputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PromptWrite { label, source } => {
                write!(formatter, "failed to write prompt for {label}: {source}")
            }
            Self::PromptFlush { label, source } => {
                write!(formatter, "failed to flush prompt for {label}: {source}")
            }
            Self::Read { label, source } => {
                write!(formatter, "failed to read {label}: {source}")
            }
            Self::Eof { label } => {
                write!(formatter, "end of input while reading {label}")
            }
            Self::InvalidInputWrite { label, source } => {
                write!(
                    formatter,
                    "failed to write invalid-input message for {label}: {source}"
                )
            }
        }
    }
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PromptWrite { source, .. }
            | Self::PromptFlush { source, .. }
            | Self::Read { source, .. }
            | Self::InvalidInputWrite { source, .. } => Some(source),
            Self::Eof { .. } => None,
        }
    }
}

pub fn try_read_value_from<T, R, W>(
    reader: &mut R,
    writer: &mut W,
    label: &'static str,
) -> Result<T, InputError>
where
    T: FromStr,
    T::Err: Display,
    R: BufRead + ?Sized,
    W: Write + ?Sized,
{
    let mut buffer = String::with_capacity(INPUT_BUFFER_CAPACITY);

    loop {
        buffer.clear();

        write!(writer, "{label} = ")
            .map_err(|source| InputError::PromptWrite { label, source })?;

        writer
            .flush()
            .map_err(|source| InputError::PromptFlush { label, source })?;

        match reader.read_line(&mut buffer) {
            Ok(0) => return Err(InputError::Eof { label }),
            Ok(_) => {}
            Err(source) if source.kind() == io::ErrorKind::Interrupted => continue,
            Err(source) => return Err(InputError::Read { label, source }),
        }

        let input = buffer.trim();

        match input.parse::<T>() {
            Ok(value) => return Ok(value),
            Err(error) => {
                writeln!(writer, "Invalid {label}: {error}")
                    .map_err(|source| InputError::InvalidInputWrite { label, source })?;

                if buffer.capacity() > MAX_RETAINED_INPUT_BUFFER_CAPACITY {
                    buffer = String::with_capacity(INPUT_BUFFER_CAPACITY);
                }
            }
        }
    }
}

/// Reads a typed value and falls back to `T::default()` on non-recoverable input errors.
///
/// Invalid parse input still retries. EOF, read failure, prompt write failure,
/// and prompt flush failure print a warning and return the type's default value.
pub fn read_value_from<T, R, W>(reader: &mut R, writer: &mut W, label: &'static str) -> T
where
    T: FromStr + Default,
    T::Err: Display,
    R: BufRead + ?Sized,
    W: Write + ?Sized,
{
    match try_read_value_from(reader, writer, label) {
        Ok(value) => value,
        Err(error) => {
            let _ = writeln!(
                writer,
                "Warning: {error}. Using default value for {label}."
            );

            T::default()
        }
    }
}

pub fn write_template_line<W>(
    writer: &mut W,
    template: &str,
    values: &[&dyn Display],
) -> io::Result<()>
where
    W: Write + ?Sized,
{
    let bytes = template.as_bytes();
    let mut cursor = 0;
    let mut literal_start = 0;
    let mut value_index = 0;
    let mut previous_written = None;
    let mut pending_template_space = false;

    while cursor < bytes.len() {
        if let Some(next_cursor) = skip_string_like(bytes, cursor) {
            cursor = next_cursor;
            continue;
        }

        if bytes[cursor] != b'{' {
            cursor += 1;
            continue;
        }

        let close = find_matching_closing_brace(template, cursor)
            .ok_or_else(|| invalid_template("unclosed template expression"))?;

        write_normalized_template_segment(
            writer,
            &template[literal_start..cursor],
            &mut previous_written,
            &mut pending_template_space,
        )?;

        let value = values
            .get(value_index)
            .ok_or_else(|| invalid_template("missing template value"))?;

        if pending_template_space
            && previous_written
                .is_some_and(|previous| should_write_pending_template_space(previous, b'x'))
        {
            writer.write_all(b" ")?;
        }

        pending_template_space = false;

        write!(writer, "{value}")?;
        previous_written = Some(b'x');

        if needs_space_after_template_value(bytes, close) {
            writer.write_all(b" ")?;
            previous_written = Some(b' ');
        }

        value_index += 1;
        cursor = close + 1;
        literal_start = cursor;
    }

    write_normalized_template_segment(
        writer,
        &template[literal_start..],
        &mut previous_written,
        &mut pending_template_space,
    )?;

    if value_index != values.len() {
        return Err(invalid_template("unused template values"));
    }

    writer.write_all(b"\n")?;

    Ok(())
}

fn write_normalized_template_segment<W>(
    writer: &mut W,
    segment: &str,
    previous_written: &mut Option<u8>,
    pending_space: &mut bool,
) -> io::Result<()>
where
    W: Write + ?Sized,
{
    let bytes = segment.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_whitespace() {
            *pending_space = true;
            cursor += 1;
            continue;
        }

        if *pending_space
            && previous_written
                .is_some_and(|previous| should_write_pending_template_space(previous, bytes[cursor]))
        {
            writer.write_all(b" ")?;
            *previous_written = Some(b' ');
        }

        *pending_space = false;

        let run_start = cursor;

        while cursor < bytes.len() && !bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        writer.write_all(&bytes[run_start..cursor])?;
        *previous_written = Some(bytes[cursor - 1]);
    }

    Ok(())
}

#[doc(hidden)]
pub fn write_template_literal_segment<W>(
    writer: &mut W,
    segment: &str,
    previous_written: &mut Option<u8>,
    pending_space: &mut bool,
    value_was_just_written: &mut bool,
) -> io::Result<()>
where
    W: Write + ?Sized,
{
    let bytes = segment.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_whitespace() {
            *pending_space = true;
            cursor += 1;
            continue;
        }

        if (*value_was_just_written
            || previous_written.is_some_and(|previous| previous == b']'))
            && is_identifier_start(bytes[cursor])
        {
            writer.write_all(b" ")?;
            *previous_written = Some(b' ');
        } else if bytes[cursor] == b'['
            && previous_written
                .is_some_and(|previous| previous.is_ascii_alphanumeric() || previous == b']')
        {
            writer.write_all(b" ")?;
            *previous_written = Some(b' ');
        } else if *pending_space
            && previous_written
                .is_some_and(|previous| should_write_pending_template_space(previous, bytes[cursor]))
        {
            writer.write_all(b" ")?;
            *previous_written = Some(b' ');
        }

        *pending_space = false;
        *value_was_just_written = false;

        let run_start = cursor;

        while cursor < bytes.len() && !bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        writer.write_all(&bytes[run_start..cursor])?;
        *previous_written = Some(bytes[cursor - 1]);
    }

    Ok(())
}

#[doc(hidden)]
pub fn write_template_value<W, Value>(
    writer: &mut W,
    value: Value,
    previous_written: &mut Option<u8>,
    pending_space: &mut bool,
    value_was_just_written: &mut bool,
) -> io::Result<()>
where
    W: Write + ?Sized,
    Value: Display,
{
    if previous_written
        .is_some_and(|previous| should_write_pending_template_space(previous, b'x'))
    {
        writer.write_all(b" ")?;
    }

    *pending_space = false;

    write!(writer, "{value}")?;

    *previous_written = Some(b'x');
    *value_was_just_written = true;

    Ok(())
}

#[doc(hidden)]
pub fn write_template_newline<W>(writer: &mut W) -> io::Result<()>
where
    W: Write + ?Sized,
{
    writer.write_all(b"\n")
}

#[doc(hidden)]
pub fn write_template_buffer<W>(writer: &mut W, buffer: &mut String) -> io::Result<()>
where
    W: Write + ?Sized,
{
    buffer.push('\n');
    writer.write_all(buffer.as_bytes())?;
    Ok(())
}

#[doc(hidden)]
pub fn append_template_literal_segment(
    buffer: &mut String,
    segment: &str,
    previous_written: &mut Option<u8>,
    pending_space: &mut bool,
    value_was_just_written: &mut bool,
) {
    let bytes = segment.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_whitespace() {
            *pending_space = true;
            cursor += 1;
            continue;
        }

        if (*value_was_just_written
            || previous_written.is_some_and(|previous| previous == b']'))
            && is_identifier_start(bytes[cursor])
        {
            buffer.push(' ');
            *previous_written = Some(b' ');
        } else if bytes[cursor] == b'['
            && previous_written
                .is_some_and(|previous| previous.is_ascii_alphanumeric() || previous == b']')
        {
            buffer.push(' ');
            *previous_written = Some(b' ');
        } else if *pending_space
            && previous_written
                .is_some_and(|previous| should_write_pending_template_space(previous, bytes[cursor]))
        {
            buffer.push(' ');
            *previous_written = Some(b' ');
        }

        *pending_space = false;
        *value_was_just_written = false;

        let run_start = cursor;

        while cursor < bytes.len() && !bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        buffer.push_str(&segment[run_start..cursor]);
        *previous_written = Some(bytes[cursor - 1]);
    }
}

#[doc(hidden)]
pub fn append_template_value<Value>(
    buffer: &mut String,
    value: Value,
    previous_written: &mut Option<u8>,
    pending_space: &mut bool,
    value_was_just_written: &mut bool,
)
where
    Value: Display,
{
    if previous_written
        .is_some_and(|previous| should_write_pending_template_space(previous, b'x'))
    {
        buffer.push(' ');
    }

    *pending_space = false;

    let _ = fmt::Write::write_fmt(buffer, format_args!("{value}"));

    *previous_written = Some(b'x');
    *value_was_just_written = true;
}

fn should_write_pending_template_space(previous: u8, next: u8) -> bool {
    !matches!(previous, b'[' | b'(' | b'{' | b'/' | b'^')
        && !matches!(next, b',' | b'.' | b':' | b';' | b']' | b')' | b'}' | b'/' | b'^')
}

// Token-based macros do not preserve source whitespace between a `{value}` group
// and a following unit word, so the writer restores that single separator.
fn needs_space_after_template_value(bytes: &[u8], close_index: usize) -> bool {
    let next_index = close_index + 1;

    next_index < bytes.len() && is_identifier_start(bytes[next_index])
}

fn invalid_template(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn find_matching_closing_brace(template: &str, open_index: usize) -> Option<usize> {
    let bytes = template.as_bytes();
    let mut cursor = open_index;
    let mut depth = 0usize;

    while cursor < bytes.len() {
        if let Some(next_cursor) = skip_string_like(bytes, cursor) {
            cursor = next_cursor;
            continue;
        }

        match bytes[cursor] {
            b'{' => {
                depth += 1;
                cursor += 1;
            }
            b'}' => {
                depth = depth.checked_sub(1)?;

                if depth == 0 {
                    return Some(cursor);
                }

                cursor += 1;
            }
            _ => {
                cursor += 1;
            }
        }
    }

    None
}

fn skip_string_like(bytes: &[u8], index: usize) -> Option<usize> {
    if let Some(next) = skip_raw_string_literal(bytes, index) {
        return Some(next);
    }

    if bytes[index] == b'"' {
        return skip_escaped_quoted_literal(bytes, index);
    }

    if is_string_prefix(bytes[index]) && index + 1 < bytes.len() && bytes[index + 1] == b'"' {
        return skip_escaped_quoted_literal(bytes, index + 1);
    }

    if bytes[index] == b'\'' {
        return skip_char_literal(bytes, index);
    }

    if bytes[index] == b'b' && index + 1 < bytes.len() && bytes[index + 1] == b'\'' {
        return skip_char_literal(bytes, index + 1);
    }

    None
}

fn skip_raw_string_literal(bytes: &[u8], index: usize) -> Option<usize> {
    let mut cursor = index;

    if cursor < bytes.len() && is_string_prefix(bytes[cursor]) {
        cursor += 1;
    }

    if cursor >= bytes.len() || bytes[cursor] != b'r' {
        return None;
    }

    cursor += 1;

    let hash_start = cursor;

    while cursor < bytes.len() && bytes[cursor] == b'#' {
        cursor += 1;
    }

    let hash_count = cursor - hash_start;

    if cursor >= bytes.len() || bytes[cursor] != b'"' {
        return None;
    }

    cursor += 1;

    while cursor < bytes.len() {
        if bytes[cursor] == b'"' {
            let mut candidate = cursor + 1;
            let mut matched_hashes = 0usize;

            while matched_hashes < hash_count && candidate < bytes.len() && bytes[candidate] == b'#'
            {
                candidate += 1;
                matched_hashes += 1;
            }

            if matched_hashes == hash_count {
                return Some(candidate);
            }
        }

        cursor += 1;
    }

    None
}

fn skip_escaped_quoted_literal(bytes: &[u8], quote_index: usize) -> Option<usize> {
    let mut cursor = quote_index + 1;
    let mut escaped = false;

    while cursor < bytes.len() {
        match bytes[cursor] {
            _ if escaped => {
                escaped = false;
                cursor += 1;
            }
            b'\\' => {
                escaped = true;
                cursor += 1;
            }
            b'"' => {
                return Some(cursor + 1);
            }
            _ => {
                cursor += 1;
            }
        }
    }

    None
}

fn skip_char_literal(bytes: &[u8], quote_index: usize) -> Option<usize> {
    if quote_index + 1 >= bytes.len() {
        return None;
    }

    let first = bytes[quote_index + 1];

    if is_identifier_start(first) {
        return None;
    }

    let mut cursor = quote_index + 1;
    let mut escaped = false;

    while cursor < bytes.len() {
        match bytes[cursor] {
            _ if escaped => {
                escaped = false;
                cursor += 1;
            }
            b'\\' => {
                escaped = true;
                cursor += 1;
            }
            b'\'' => {
                return Some(cursor + 1);
            }
            _ => {
                cursor += 1;
            }
        }
    }

    None
}

fn is_string_prefix(byte: u8) -> bool {
    matches!(byte, b'b' | b'c')
}

fn is_identifier_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic()
}

#[macro_export]
macro_rules! input {
    (
        $($items:tt)+
    ) => {
        $crate::__input_declare! {
            $($items)+
        }

        {
            let __stdin_handle = ::std::io::stdin();
            let mut __stdin_lock = __stdin_handle.lock();

            let __stdout_handle = ::std::io::stdout();
            let mut __stdout_lock = __stdout_handle.lock();

            $crate::__input_read! {
                reader: &mut __stdin_lock,
                writer: &mut __stdout_lock,
                $($items)+
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __input_declare {
    () => {};

    (,) => {};

    (
        let mut $name:ident : $ty:ty, $($rest:tt)*
    ) => {
        let mut $name: $ty;

        $crate::__input_declare! {
            $($rest)*
        }
    };

    (
        let mut $name:ident : $ty:ty
    ) => {
        let mut $name: $ty;
    };

    (
        let mut $name:ident, $($rest:tt)*
    ) => {
        let mut $name;

        $crate::__input_declare! {
            $($rest)*
        }
    };

    (
        let mut $name:ident
    ) => {
        let mut $name;
    };

    (
        let $name:ident : $ty:ty, $($rest:tt)*
    ) => {
        let $name: $ty;

        $crate::__input_declare! {
            $($rest)*
        }
    };

    (
        let $name:ident : $ty:ty
    ) => {
        let $name: $ty;
    };

    (
        let $name:ident, $($rest:tt)*
    ) => {
        let $name;

        $crate::__input_declare! {
            $($rest)*
        }
    };

    (
        let $name:ident
    ) => {
        let $name;
    };

    (
        const $name:ident $($rest:tt)*
    ) => {
        ::std::compile_error!(
            "input! does not support `const`. Input is read at runtime. Use `let name: Type` or an existing variable instead."
        );
    };

    (
        $name:ident : $ty:ty, $($rest:tt)*
    ) => {
        let $name: $ty;

        $crate::__input_declare! {
            $($rest)*
        }
    };

    (
        $name:ident : $ty:ty
    ) => {
        let $name: $ty;
    };

    (
        $name:ident, $($rest:tt)*
    ) => {
        $crate::__input_declare! {
            $($rest)*
        }
    };

    (
        $name:ident
    ) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __input_read {
    (
        reader: $reader:expr,
        writer: $writer:expr,
    ) => {};

    (
        reader: $reader:expr,
        writer: $writer:expr,
        ,
    ) => {};

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let mut $name:ident : $ty:ty, $($rest:tt)*
    ) => {
        $name = $crate::read_value_from::<$ty, _, _>(
            $reader,
            $writer,
            stringify!($name),
        );

        $crate::__input_read! {
            reader: $reader,
            writer: $writer,
            $($rest)*
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let mut $name:ident : $ty:ty
    ) => {
        $name = $crate::read_value_from::<$ty, _, _>(
            $reader,
            $writer,
            stringify!($name),
        );
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let mut $name:ident, $($rest:tt)*
    ) => {
        $name = $crate::read_value_from(
            $reader,
            $writer,
            stringify!($name),
        );

        $crate::__input_read! {
            reader: $reader,
            writer: $writer,
            $($rest)*
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let mut $name:ident
    ) => {
        $name = $crate::read_value_from(
            $reader,
            $writer,
            stringify!($name),
        );
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let $name:ident : $ty:ty, $($rest:tt)*
    ) => {
        $name = $crate::read_value_from::<$ty, _, _>(
            $reader,
            $writer,
            stringify!($name),
        );

        $crate::__input_read! {
            reader: $reader,
            writer: $writer,
            $($rest)*
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let $name:ident : $ty:ty
    ) => {
        $name = $crate::read_value_from::<$ty, _, _>(
            $reader,
            $writer,
            stringify!($name),
        );
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let $name:ident, $($rest:tt)*
    ) => {
        $name = $crate::read_value_from(
            $reader,
            $writer,
            stringify!($name),
        );

        $crate::__input_read! {
            reader: $reader,
            writer: $writer,
            $($rest)*
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        let $name:ident
    ) => {
        $name = $crate::read_value_from(
            $reader,
            $writer,
            stringify!($name),
        );
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        const $name:ident $($rest:tt)*
    ) => {
        ::std::compile_error!(
            "input! does not support `const`. Input is read at runtime. Use `let name: Type` or an existing variable instead."
        );
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        $name:ident : $ty:ty, $($rest:tt)*
    ) => {
        $name = $crate::read_value_from::<$ty, _, _>(
            $reader,
            $writer,
            stringify!($name),
        );

        $crate::__input_read! {
            reader: $reader,
            writer: $writer,
            $($rest)*
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        $name:ident : $ty:ty
    ) => {
        $name = $crate::read_value_from::<$ty, _, _>(
            $reader,
            $writer,
            stringify!($name),
        );
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        $name:ident, $($rest:tt)*
    ) => {
        $name = $crate::read_value_from(
            $reader,
            $writer,
            stringify!($name),
        );

        $crate::__input_read! {
            reader: $reader,
            writer: $writer,
            $($rest)*
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        $name:ident
    ) => {
        $name = $crate::read_value_from(
            $reader,
            $writer,
            stringify!($name),
        );
    };
}

#[macro_export]
macro_rules! input_from {
    (
        reader: $reader:expr,
        writer: $writer:expr,
        $($items:tt)+
    ) => {
        $crate::__input_declare! {
            $($items)+
        }

        {
            let mut __reader = $reader;
            let mut __writer = $writer;

            $crate::__input_read! {
                reader: &mut __reader,
                writer: &mut __writer,
                $($items)+
            }
        }
    };
}

#[macro_export]
macro_rules! output {
    (
        $(
            $tokens:tt
        )*
    ) => {{
        let __stdout_handle = ::std::io::stdout();
        let mut __stdout_lock = __stdout_handle.lock();

        $crate::output_buffered_to! {
            writer: &mut __stdout_lock,
            $($tokens)*
        }
    }};
}

#[macro_export]
macro_rules! output_to {
    (
        writer: $writer:expr,
    ) => {{}};

    (
        writer: $writer:expr,
        << $($tokens:tt)*
    ) => {{
        let mut __writer = $writer;

        $crate::__output_arrow_lines! {
            writer: __writer,
            current: [],
            scan: $($tokens)*
        }
    }};
}

#[macro_export]
macro_rules! output_buffered_to {
    (
        writer: $writer:expr,
    ) => {{}};

    (
        writer: $writer:expr,
        << $($tokens:tt)*
    ) => {{
        let mut __writer = $writer;
        let mut __output_buffer = ::std::string::String::with_capacity(512);

        $crate::__output_buffered_arrow_lines! {
            writer: __writer,
            buffer: __output_buffer,
            current: [],
            scan: $($tokens)*
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_arrow_lines {
    (
        writer: $writer:ident,
        current: [],
        scan:
    ) => {};

    (
        writer: $writer:ident,
        current: [$($line:tt)+],
        scan:
    ) => {
        $crate::__output_direct_one_line! {
            writer: $writer,
            $($line)+
        }
    };

    (
        writer: $writer:ident,
        current: [],
        scan: << $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)+],
        scan: << $($rest:tt)*
    ) => {
        $crate::__output_direct_one_line! {
            writer: $writer,
            $($line)+
        }

        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: [$($group:tt)*] $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* [$($group)*]],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: { $($group:tt)* } $next:ident $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* {$($group)*} $next],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: { $($group:tt)* } $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* {$($group)*}],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: ( $($group:tt)* ) $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* ( $($group)* )],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: $first:ident $second:ident $third:ident $fourth:ident $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* $first $second $third $fourth],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: $first:ident $second:ident $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* $first $second],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: $next:ident $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* $next],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: $literal:literal $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* $literal],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        scan: $next:tt $($rest:tt)*
    ) => {
        $crate::__output_arrow_lines! {
            writer: $writer,
            current: [$($line)* $next],
            scan: $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_buffered_arrow_lines {
    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [],
        scan:
    ) => {};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)+],
        scan:
    ) => {
        $crate::__output_one_line! {
            writer: $writer,
            buffer: $buffer,
            $($line)+
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [],
        scan: << $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)+],
        scan: << $($rest:tt)*
    ) => {
        $crate::__output_one_line! {
            writer: $writer,
            buffer: $buffer,
            $($line)+
        }

        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: [$($group:tt)*] $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* [$($group)*]],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: { $($group:tt)* } $next:ident $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* {$($group)*} $next],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: { $($group:tt)* } $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* {$($group)*}],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: ( $($group:tt)* ) $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* ( $($group)* )],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: $first:ident $second:ident $third:ident $fourth:ident $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* $first $second $third $fourth],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: $first:ident $second:ident $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* $first $second],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: $next:ident $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* $next],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: $literal:literal $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* $literal],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: _ $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* _],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: = $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* =],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: , $next:ident $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* , $next],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: , $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* ,],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: . $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* .],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: : $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* :],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: ; $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* ;],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: / $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* /],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: ^ $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* ^],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: - $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* -],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: + $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* +],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: * $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* *],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: % $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* %],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: ! $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* !],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: ? $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* ?],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: @ $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* @],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: # $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* #],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: & $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* &],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: | $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* |],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        current: [$($line:tt)*],
        scan: > $($rest:tt)*
    ) => {
        $crate::__output_buffered_arrow_lines! {
            writer: $writer,
            buffer: $buffer,
            current: [$($line)* >],
            scan: $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_direct_one_line {
    (
        writer: $writer:ident,
        $($line:tt)+
    ) => {{
        let mut __previous_written = None;
        let mut __pending_space = false;
        let mut __value_was_just_written = false;

        $crate::__output_direct_line_parts! {
            writer: $writer,
            previous_written: __previous_written,
            pending_space: __pending_space,
            value_was_just_written: __value_was_just_written,
            literal: [],
            scan: $($line)+
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_direct_line_parts {
    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan:
    ) => {{
        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        let _ = $crate::write_template_newline(&mut $writer);
    }};

    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: { $value:expr } $($rest:tt)*
    ) => {{
        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        let _ = $crate::write_template_value(
            &mut $writer,
            $value,
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_direct_line_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: [$($group:tt)*] $($rest:tt)*
    ) => {{
        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            "[",
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_direct_fragment_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($group)*
        }

        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            "]",
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_direct_line_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: ( $($group:tt)* ) $($rest:tt)*
    ) => {
        $crate::__output_direct_line_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* ( $($group)* )],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: $next:tt $($rest:tt)*
    ) => {
        $crate::__output_direct_line_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* $next],
            scan: $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_direct_fragment_parts {
    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan:
    ) => {{
        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );
    }};

    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: { $value:expr } $($rest:tt)*
    ) => {{
        let _ = $crate::write_template_literal_segment(
            &mut $writer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        let _ = $crate::write_template_value(
            &mut $writer,
            $value,
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_direct_fragment_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: $next:tt $($rest:tt)*
    ) => {
        $crate::__output_direct_fragment_parts! {
            writer: $writer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* $next],
            scan: $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_one_line {
    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        $($line:tt)+
    ) => {{
        $buffer.clear();
        let mut __previous_written = None;
        let mut __pending_space = false;
        let mut __value_was_just_written = false;

        $crate::__output_write_line_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: __previous_written,
            pending_space: __pending_space,
            value_was_just_written: __value_was_just_written,
            literal: [],
            scan: $($line)+
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_write_line_parts {
    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan:
    ) => {{
        $crate::append_template_literal_segment(
            &mut $buffer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        let _ = $crate::write_template_buffer(&mut $writer, &mut $buffer);
    }};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: { $value:expr } $($rest:tt)*
    ) => {{
        $crate::append_template_literal_segment(
            &mut $buffer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::append_template_value(
            &mut $buffer,
            $value,
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_write_line_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: [$($group:tt)*] $($rest:tt)*
    ) => {{
        $crate::append_template_literal_segment(
            &mut $buffer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::append_template_literal_segment(
            &mut $buffer,
            "[",
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_write_fragment_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($group)*
        }

        $crate::append_template_literal_segment(
            &mut $buffer,
            "]",
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_write_line_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: ( $($group:tt)* ) $($rest:tt)*
    ) => {
        $crate::__output_write_line_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* ( $($group)* )],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: $next:tt $($rest:tt)*
    ) => {
        $crate::__output_write_line_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* $next],
            scan: $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_write_fragment_parts {
    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan:
    ) => {{
        $crate::append_template_literal_segment(
            &mut $buffer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );
    }};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: { $value:expr } $($rest:tt)*
    ) => {{
        $crate::append_template_literal_segment(
            &mut $buffer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::append_template_value(
            &mut $buffer,
            $value,
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_write_fragment_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: [$($group:tt)*] $($rest:tt)*
    ) => {{
        $crate::append_template_literal_segment(
            &mut $buffer,
            stringify!($($literal)*),
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::append_template_literal_segment(
            &mut $buffer,
            "[",
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_write_fragment_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($group)*
        }

        $crate::append_template_literal_segment(
            &mut $buffer,
            "]",
            &mut $previous_written,
            &mut $pending_space,
            &mut $value_was_just_written,
        );

        $crate::__output_write_fragment_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [],
            scan: $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: ( $($group:tt)* ) $($rest:tt)*
    ) => {
        $crate::__output_write_fragment_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* ( $($group)* )],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        buffer: $buffer:ident,
        previous_written: $previous_written:ident,
        pending_space: $pending_space:ident,
        value_was_just_written: $value_was_just_written:ident,
        literal: [$($literal:tt)*],
        scan: $next:tt $($rest:tt)*
    ) => {
        $crate::__output_write_fragment_parts! {
            writer: $writer,
            buffer: $buffer,
            previous_written: $previous_written,
            pending_space: $pending_space,
            value_was_just_written: $value_was_just_written,
            literal: [$($literal)* $next],
            scan: $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_collect_values {
    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan:
    ) => {{
        let __values: &[&dyn ::std::fmt::Display] = &[
            $($values,)*
        ];

        let _ = $crate::write_template_line(
            &mut $writer,
            $template,
            __values,
        );
    }};

    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan: [$($ignored:tt)*] $($rest:tt)*
    ) => {
        $crate::__output_collect_values! {
            writer: $writer,
            template: $template,
            values: [$($values,)*],
            scan: $($ignored)* $($rest)*
        }
    };

    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan: ( $($ignored:tt)* ) $($rest:tt)*
    ) => {
        $crate::__output_collect_values! {
            writer: $writer,
            template: $template,
            values: [$($values,)*],
            scan: $($ignored)* $($rest)*
        }
    };

    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan: { $value:expr } $($rest:tt)*
    ) => {
        $crate::__output_collect_values! {
            writer: $writer,
            template: $template,
            values: [
                $($values,)*
                &($value) as &dyn ::std::fmt::Display,
            ],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan: $first:ident $second:ident $third:ident $fourth:ident $($rest:tt)*
    ) => {
        $crate::__output_collect_values! {
            writer: $writer,
            template: $template,
            values: [$($values,)*],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan: $first:ident $second:ident $($rest:tt)*
    ) => {
        $crate::__output_collect_values! {
            writer: $writer,
            template: $template,
            values: [$($values,)*],
            scan: $($rest)*
        }
    };

    (
        writer: $writer:ident,
        template: $template:expr,
        values: [$($values:expr,)*],
        scan: $ignored:tt $($rest:tt)*
    ) => {
        $crate::__output_collect_values! {
            writer: $writer,
            template: $template,
            values: [$($values,)*],
            scan: $($rest)*
        }
    };
}




