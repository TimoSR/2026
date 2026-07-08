pub use io_macros_project_macros::{output, output_to};

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

