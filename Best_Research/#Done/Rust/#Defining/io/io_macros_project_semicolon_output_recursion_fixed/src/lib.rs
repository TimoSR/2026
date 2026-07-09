use std::fmt::{self, Display};
use std::io::{self, BufRead, Write};
use std::str::FromStr;

const INPUT_BUFFER_CAPACITY: usize = 64;
const MAX_RETAINED_INPUT_BUFFER_CAPACITY: usize = 8 * 1024;

#[derive(Debug)]
pub enum InputError {
    PromptWrite { label: &'static str, source: io::Error },
    PromptFlush { label: &'static str, source: io::Error },
    Read { label: &'static str, source: io::Error },
    Eof { label: &'static str },
    InvalidInputWrite { label: &'static str, source: io::Error },
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
                write!(formatter, "failed to write invalid-input message for {label}: {source}")
            }
        }
    }
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
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

    loop {
        buffer.clear();

        write!(writer, "{label} = ").map_err(|source| InputError::PromptWrite { label, source })?;

        writer.flush().map_err(|source| InputError::PromptFlush { label, source })?;

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
                writeln!(writer, "Invalid {label}: {error}").map_err(|source| InputError::InvalidInputWrite { label, source })?;

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
            let _ = writeln!(writer, "Warning: {error}. Using default value for {label}.");

            T::default()
        }
    }
}

pub trait OutputValue {
    fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized;
}

macro_rules! implement_output_value_with_display {
    ($($value_type:ty),+ $(,)?) => {
        $(
            impl OutputValue for $value_type {
                fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
                where
                    W: Write + ?Sized,
                {
                    write!(writer, "{self}")
                }
            }
        )+
    };
}

implement_output_value_with_display!(
    bool, char, str, String, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
    f32, f64,
);

impl<Value> OutputValue for &Value
where
    Value: OutputValue + ?Sized,
{
    fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        (*self).write_output_value(writer)
    }
}

impl<Value> OutputValue for &mut Value
where
    Value: OutputValue + ?Sized,
{
    fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        (**self).write_output_value(writer)
    }
}

impl<OutputItem> OutputValue for [OutputItem]
where
    OutputItem: OutputValue,
{
    fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        writer.write_all(b"[")?;

        let mut index = 0;

        while index < self.len() {
            if index > 0 {
                writer.write_all(b", ")?;
            }

            self[index].write_output_value(writer)?;
            index += 1;
        }

        writer.write_all(b"]")
    }
}

impl<OutputItem, const ITEM_COUNT: usize> OutputValue for [OutputItem; ITEM_COUNT]
where
    OutputItem: OutputValue,
{
    fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.as_slice().write_output_value(writer)
    }
}

impl<OutputItem> OutputValue for Vec<OutputItem>
where
    OutputItem: OutputValue,
{
    fn write_output_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.as_slice().write_output_value(writer)
    }
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
            && previous_written.is_some_and(|previous| {
                should_write_pending_template_space(previous, bytes[cursor])
            })
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
    value: &Value,
    previous_written: &mut Option<u8>,
    pending_space: &mut bool,
    value_was_just_written: &mut bool,
) -> io::Result<()>
where
    W: Write + ?Sized,
    Value: OutputValue + ?Sized,
{
    if previous_written.is_some_and(|previous| should_write_pending_template_space(previous, b'x')) {
        writer.write_all(b" ")?;
    }

    *pending_space = false;

    value.write_output_value(writer)?;

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

fn should_write_pending_template_space(previous: u8, next: u8) -> bool {
    !matches!(previous, b'[' | b'(' | b'{' | b'/' | b'^') && !matches!(next, b',' | b'.' | b':' | b';' | b'!' | b'?' | b']' | b')' | b'}' | b'/' | b'^')
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
        let mut __output_buffer: ::std::vec::Vec<u8> = ::std::vec::Vec::with_capacity(2048);

        $crate::output_to! {
            writer: &mut __output_buffer,
            << $($tokens)*
        }

        let _ = ::std::io::Write::write_all(&mut __writer, &__output_buffer);
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
            &($value),
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
            &($value),
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
