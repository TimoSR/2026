use std::fmt::Display;
use std::io::{BufRead, Write};
use std::str::FromStr;

/// Reads a typed value from `reader`, prompting with `label`.
///
/// Invalid parse input causes a retry.
/// Read/write errors are treated as non-critical for teaching/demo ergonomics.
pub fn read_value_from<T, R, W>(
    reader: &mut R,
    writer: &mut W,
    label: &'static str,
) -> T
where
    T: FromStr,
    T::Err: Display,
    R: BufRead,
    W: Write,
{
    let mut buffer = String::with_capacity(64);

    loop {
        buffer.clear();

        let _ = write!(writer, "{label} = ");
        let _ = writer.flush();

        let Ok(bytes_read) = reader.read_line(&mut buffer) else {
            let _ = writeln!(writer, "Failed to read {label}. Try again.");
            continue;
        };

        if bytes_read == 0 {
            let _ = writeln!(writer, "No input received for {label}. Try again.");
            continue;
        }

        let input = buffer.trim();

        match input.parse::<T>() {
            Ok(value) => return value,
            Err(error) => {
                let _ = writeln!(writer, "Invalid {label}: {error}");
            }
        }
    }
}

/// Assigns typed stdin values into variables that must already exist.
///
/// Example:
///
/// ```rust
/// use io_macros_project::input;
///
/// let mut distance = 0.0_f64;
/// let mut time = 0.0_f64;
///
/// input! {
///     distance,
///     time,
/// }
/// ```
#[macro_export]
macro_rules! input {
    (
        $(
            $name:ident
        ),+ $(,)?
    ) => {{
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        $(
            $name = $crate::read_value_from(
                &mut stdin,
                &mut stdout,
                stringify!($name),
            );
        )+
    }};
}

/// Assigns typed values into existing variables using a custom reader/writer.
///
/// This is useful for tests and non-stdin input sources.
#[macro_export]
macro_rules! input_from {
    (
        reader: $reader:expr,
        writer: $writer:expr,
        $(
            $name:ident
        ),+ $(,)?
    ) => {{
        $(
            $name = $crate::read_value_from(
                $reader,
                $writer,
                stringify!($name),
            );
        )+
    }};
}

/// Prints labeled values using a C#-style call-site shape.
///
/// Example:
///
/// ```rust
/// use io_macros_project::output;
///
/// let distance = 42.0;
///
/// output! {
///     distance = {distance}
/// }
/// ```
///
/// Output errors are ignored intentionally:
/// - no `?`
/// - no panic
/// - no returned `Result`
#[macro_export]
macro_rules! output {
    (
        $(
            $label:ident = { $value:expr }
        )*
    ) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        $(
            let _ = writeln!(
                stdout,
                "{} = {}",
                stringify!($label),
                $value
            );
        )*
    }};
}

/// Writes labeled values to a custom writer.
///
/// This variant is testable and still intentionally ignores write failures.
#[macro_export]
macro_rules! output_to {
    (
        writer: $writer:expr,
        $(
            $label:ident = { $value:expr }
        )*
    ) => {{
        use std::io::Write;

        $(
            let _ = writeln!(
                $writer,
                "{} = {}",
                stringify!($label),
                $value
            );
        )*
    }};
}
