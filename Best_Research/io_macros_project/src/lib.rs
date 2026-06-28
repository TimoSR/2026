use std::fmt::{self, Display};
use std::io::{self, BufRead, Write};
use std::str::FromStr;

const INPUT_BUFFER_CAPACITY: usize = 64;
const MAX_RETAINED_INPUT_BUFFER_CAPACITY: usize = 8 * 1024;

#[derive(Debug)]
pub enum InputError
{
    PromptWrite
    {
        label: &'static str, source: io::Error
    },
    PromptFlush
    {
        label: &'static str, source: io::Error
    },
    Read
    {
        label: &'static str, source: io::Error
    },
    Eof
    {
        label: &'static str
    },
    InvalidInputWrite
    {
        label: &'static str, source: io::Error
    },
}

impl fmt::Display for InputError
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Self::PromptWrite { label, source } =>
            {
                write!(formatter, "failed to write prompt for {label}: {source}")
            }
            Self::PromptFlush { label, source } =>
            {
                write!(formatter, "failed to flush prompt for {label}: {source}")
            }
            Self::Read { label, source } =>
            {
                write!(formatter, "failed to read {label}: {source}")
            }
            Self::Eof { label } =>
            {
                write!(formatter, "end of input while reading {label}")
            }
            Self::InvalidInputWrite { label, source } =>
            {
                write!(formatter, "failed to write invalid-input message for {label}: {source}")
            }
        }
    }
}

impl std::error::Error for InputError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        match self
        {
            Self::PromptWrite { source, .. } | Self::PromptFlush { source, .. } | Self::Read { source, .. } | Self::InvalidInputWrite { source, .. } => Some(source),
            Self::Eof { .. } => None,
        }
    }
}

pub fn try_read_value_from<T, R, W>(reader: &mut R, writer: &mut W, label: &'static str) -> Result<T, InputError>
where
    T: FromStr,
    T::Err: Display,
    R: BufRead + ?Sized,
    W: Write + ?Sized,
{
    let mut buffer = String::with_capacity(INPUT_BUFFER_CAPACITY);

    loop
    {
        buffer.clear();

        write!(writer, "{label} = ").map_err(|source| InputError::PromptWrite { label, source })?;

        writer.flush().map_err(|source| InputError::PromptFlush { label, source })?;

        match reader.read_line(&mut buffer)
        {
            Ok(0) => return Err(InputError::Eof { label }),
            Ok(_) =>
            {}
            Err(source) if source.kind() == io::ErrorKind::Interrupted => continue,
            Err(source) => return Err(InputError::Read { label, source }),
        }

        let input = buffer.trim();

        match input.parse::<T>()
        {
            Ok(value) => return Ok(value),
            Err(error) =>
            {
                writeln!(writer, "Invalid {label}: {error}").map_err(|source| InputError::InvalidInputWrite { label, source })?;

                if buffer.capacity() > MAX_RETAINED_INPUT_BUFFER_CAPACITY
                {
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
    match try_read_value_from(reader, writer, label)
    {
        Ok(value) => value,
        Err(error) =>
        {
            let _ = writeln!(writer, "Warning: {error}. Using default value for {label}.");

            T::default()
        }
    }
}

pub fn write_template_line<W>(writer: &mut W, template: &str, values: &[&dyn Display]) -> io::Result<()>
where
    W: Write + ?Sized,
{
    let bytes = template.as_bytes();
    let mut cursor = 0;
    let mut literal_start = 0;
    let mut value_index = 0;

    while cursor < bytes.len()
    {
        if let Some(next_cursor) = skip_string_like(bytes, cursor)
        {
            cursor = next_cursor;
            continue;
        }

        if bytes[cursor] != b'{'
        {
            cursor += 1;
            continue;
        }

        let close = find_matching_closing_brace(template, cursor).ok_or_else(|| invalid_template("unclosed template expression"))?;

        writer.write_all(template[literal_start..cursor].as_bytes())?;

        let value = values.get(value_index).ok_or_else(|| invalid_template("missing template value"))?;

        write!(writer, "{value}")?;

        value_index += 1;
        cursor = close + 1;
        literal_start = cursor;
    }

    writer.write_all(template[literal_start..].as_bytes())?;

    if value_index != values.len()
    {
        return Err(invalid_template("unused template values"));
    }

    writer.write_all(b"\n")?;

    Ok(())
}

fn invalid_template(message: &'static str) -> io::Error
{
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn find_matching_closing_brace(template: &str, open_index: usize) -> Option<usize>
{
    let bytes = template.as_bytes();
    let mut cursor = open_index;
    let mut depth = 0usize;

    while cursor < bytes.len()
    {
        if let Some(next_cursor) = skip_string_like(bytes, cursor)
        {
            cursor = next_cursor;
            continue;
        }

        match bytes[cursor]
        {
            b'{' =>
            {
                depth += 1;
                cursor += 1;
            }
            b'}' =>
            {
                depth = depth.checked_sub(1)?;

                if depth == 0
                {
                    return Some(cursor);
                }

                cursor += 1;
            }
            _ =>
            {
                cursor += 1;
            }
        }
    }

    None
}

fn skip_string_like(bytes: &[u8], index: usize) -> Option<usize>
{
    if let Some(next) = skip_raw_string_literal(bytes, index)
    {
        return Some(next);
    }

    if bytes[index] == b'"'
    {
        return skip_escaped_quoted_literal(bytes, index);
    }

    if is_string_prefix(bytes[index]) && index + 1 < bytes.len() && bytes[index + 1] == b'"'
    {
        return skip_escaped_quoted_literal(bytes, index + 1);
    }

    if bytes[index] == b'\''
    {
        return skip_char_literal(bytes, index);
    }

    if bytes[index] == b'b' && index + 1 < bytes.len() && bytes[index + 1] == b'\''
    {
        return skip_char_literal(bytes, index + 1);
    }

    None
}

fn skip_raw_string_literal(bytes: &[u8], index: usize) -> Option<usize>
{
    let mut cursor = index;

    if cursor < bytes.len() && is_string_prefix(bytes[cursor])
    {
        cursor += 1;
    }

    if cursor >= bytes.len() || bytes[cursor] != b'r'
    {
        return None;
    }

    cursor += 1;

    let hash_start = cursor;

    while cursor < bytes.len() && bytes[cursor] == b'#'
    {
        cursor += 1;
    }

    let hash_count = cursor - hash_start;

    if cursor >= bytes.len() || bytes[cursor] != b'"'
    {
        return None;
    }

    cursor += 1;

    while cursor < bytes.len()
    {
        if bytes[cursor] == b'"'
        {
            let mut candidate = cursor + 1;
            let mut matched_hashes = 0usize;

            while matched_hashes < hash_count && candidate < bytes.len() && bytes[candidate] == b'#'
            {
                candidate += 1;
                matched_hashes += 1;
            }

            if matched_hashes == hash_count
            {
                return Some(candidate);
            }
        }

        cursor += 1;
    }

    None
}

fn skip_escaped_quoted_literal(bytes: &[u8], quote_index: usize) -> Option<usize>
{
    let mut cursor = quote_index + 1;
    let mut escaped = false;

    while cursor < bytes.len()
    {
        match bytes[cursor]
        {
            _ if escaped =>
            {
                escaped = false;
                cursor += 1;
            }
            b'\\' =>
            {
                escaped = true;
                cursor += 1;
            }
            b'"' =>
            {
                return Some(cursor + 1);
            }
            _ =>
            {
                cursor += 1;
            }
        }
    }

    None
}

fn skip_char_literal(bytes: &[u8], quote_index: usize) -> Option<usize>
{
    if quote_index + 1 >= bytes.len()
    {
        return None;
    }

    let first = bytes[quote_index + 1];

    if is_identifier_start(first)
    {
        return None;
    }

    let mut cursor = quote_index + 1;
    let mut escaped = false;

    while cursor < bytes.len()
    {
        match bytes[cursor]
        {
            _ if escaped =>
            {
                escaped = false;
                cursor += 1;
            }
            b'\\' =>
            {
                escaped = true;
                cursor += 1;
            }
            b'\'' =>
            {
                return Some(cursor + 1);
            }
            _ =>
            {
                cursor += 1;
            }
        }
    }

    None
}

fn is_string_prefix(byte: u8) -> bool
{
    matches!(byte, b'b' | b'c')
}

fn is_identifier_start(byte: u8) -> bool
{
    byte == b'_' || byte.is_ascii_alphabetic()
}

#[macro_export]
macro_rules! input {
    (
        $(
            $name:ident : $ty:ty
        ),+ $(,)?
    ) => {
        $(
            let $name: $ty;
        )+

        {
            let __stdin_handle = ::std::io::stdin();
            let mut __stdin_lock = __stdin_handle.lock();

            let __stdout_handle = ::std::io::stdout();
            let mut __stdout_lock = __stdout_handle.lock();

            $(
                $name = $crate::read_value_from::<$ty, _, _>(
                    &mut __stdin_lock,
                    &mut __stdout_lock,
                    stringify!($name),
                );
            )+
        }
    };

    (
        $(
            $name:ident
        ),+ $(,)?
    ) => {{
        let __stdin_handle = ::std::io::stdin();
        let mut __stdin_lock = __stdin_handle.lock();

        let __stdout_handle = ::std::io::stdout();
        let mut __stdout_lock = __stdout_handle.lock();

        $(
            $name = $crate::read_value_from(
                &mut __stdin_lock,
                &mut __stdout_lock,
                stringify!($name),
            );
        )+
    }};
}

#[macro_export]
macro_rules! input_from {
    (
        reader: $reader:expr,
        writer: $writer:expr,
        $(
            $name:ident : $ty:ty
        ),+ $(,)?
    ) => {
        $(
            let $name: $ty;
        )+

        {
            let mut __reader = $reader;
            let mut __writer = $writer;

            $(
                $name = $crate::read_value_from::<$ty, _, _>(
                    &mut __reader,
                    &mut __writer,
                    stringify!($name),
                );
            )+
        }
    };

    (
        reader: $reader:expr,
        writer: $writer:expr,
        $(
            $name:ident
        ),+ $(,)?
    ) => {{
        let mut __reader = $reader;
        let mut __writer = $writer;

        $(
            $name = $crate::read_value_from(
                &mut __reader,
                &mut __writer,
                stringify!($name),
            );
        )+
    }};
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

        $crate::output_to! {
            writer: &mut __stdout_lock,
            $($tokens)*
        }
    }};
}

#[macro_export]
macro_rules! output_to {
    (
        writer: $writer:expr,
        $(
            $tokens:tt
        )*
    ) => {{
        let mut __writer = $writer;

        $crate::__output_lines! {
            writer: __writer,
            current: [],
            $($tokens)*
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_lines {
    (
        writer: $writer:ident,
        current: [],
    ) => {};

    (
        writer: $writer:ident,
        current: [$($line:tt)+],
    ) => {{
        $crate::__output_one_line! {
            writer: $writer,
            $($line)+
        }
    }};

    (
        writer: $writer:ident,
        current: [],
        ,
        $($rest:tt)*
    ) => {{
        $crate::__output_lines! {
            writer: $writer,
            current: [],
            $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        current: [$($line:tt)+],
        ,
        $($rest:tt)*
    ) => {{
        $crate::__output_one_line! {
            writer: $writer,
            $($line)+
        }

        $crate::__output_lines! {
            writer: $writer,
            current: [],
            $($rest)*
        }
    }};

    (
        writer: $writer:ident,
        current: [$($line:tt)*],
        $next:tt
        $($rest:tt)*
    ) => {
        $crate::__output_lines! {
            writer: $writer,
            current: [$($line)* $next],
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __output_one_line {
    (
        writer: $writer:ident,
        $($line:tt)+
    ) => {{
        $crate::__output_collect_values! {
            writer: $writer,
            template: stringify!($($line)+),
            values: [],
            scan: $($line)+
        }
    }};
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
