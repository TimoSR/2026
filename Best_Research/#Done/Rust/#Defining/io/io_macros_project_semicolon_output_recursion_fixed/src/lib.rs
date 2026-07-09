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
    fn write_output_value<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized;
}

impl<Value> OutputValue for &&Value
where
    Value: Display + ?Sized,
{
    fn write_output_value<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        write!(writer, "{}", *self)
    }
}

impl<OutputItem> OutputValue for &[OutputItem]
where
    OutputItem: Display,
{
    fn write_output_value<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        writer.write_all(b"[")?;

        let mut index = 0;

        while index < self.len() {
            if index > 0 {
                writer.write_all(b", ")?;
            }

            write!(writer, "{}", &self[index])?;
            index += 1;
        }

        writer.write_all(b"]")
    }
}

impl<OutputItem, const ITEM_COUNT: usize> OutputValue for &[OutputItem; ITEM_COUNT]
where
    OutputItem: Display,
{
    fn write_output_value<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.as_slice().write_output_value(writer)
    }
}

impl<OutputItem> OutputValue for &Vec<OutputItem>
where
    OutputItem: Display,
{
    fn write_output_value<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.as_slice().write_output_value(writer)
    }
}

#[doc(hidden)]
pub fn write_template_literal_segment<W>(writer: &mut W, segment: &str, previous_written: &mut Option<u8>, pending_space: &mut bool, value_was_just_written: &mut bool) -> io::Result<()>
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

        let needs_space_after_value_or_bracket = (*value_was_just_written || previous_written.is_some_and(|previous| previous == b']')) && is_identifier_start(bytes[cursor]);
        let needs_space_before_bracket = bytes[cursor] == b'[' && previous_written.is_some_and(|previous| previous.is_ascii_alphanumeric() || previous == b']');
        let needs_pending_template_space = *pending_space && previous_written.is_some_and(|previous| should_write_pending_template_space(previous, bytes[cursor]));

        if needs_space_after_value_or_bracket || needs_space_before_bracket || needs_pending_template_space {
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
pub fn write_template_newline<W>(writer: &mut W) -> io::Result<()>
where
    W: Write + ?Sized,
{
    writer.write_all(b"\n")
}

#[doc(hidden)]
pub fn should_write_pending_template_space(previous: u8, next: u8) -> bool {
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

pub use io_macros_project_macros::{output, output_buffered_to, output_to};
