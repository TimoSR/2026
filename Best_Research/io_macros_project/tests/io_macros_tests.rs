use std::io::Cursor;

use io_macros_project::{input_from, output_to, read_value_from, try_read_value_from, InputError};

#[test]
fn try_read_value_retries_until_valid_input() {
    let mut reader = Cursor::new(b"not-a-number\n42\n");
    let mut writer = Vec::new();

    let value: i32 = try_read_value_from(&mut reader, &mut writer, "answer")
        .expect("valid fallback input should parse");

    assert_eq!(value, 42);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert!(output.contains("answer = "));
    assert!(output.contains("Invalid answer"));
    assert!(!output.contains("Warning:"));
}

#[test]
fn try_read_value_returns_eof_error_without_looping_forever() {
    let mut reader = Cursor::new(b"");
    let mut writer = Vec::new();

    let result: Result<i32, InputError> = try_read_value_from(&mut reader, &mut writer, "answer");

    assert!(matches!(result, Err(InputError::Eof { label: "answer" })));

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "answer = ");
}

#[test]
fn read_value_warns_and_returns_default_on_eof() {
    let mut reader = Cursor::new(b"");
    let mut writer = Vec::new();

    let value: i32 = read_value_from(&mut reader, &mut writer, "answer");

    assert_eq!(value, 0);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert!(output.contains("answer = "));
    assert!(output.contains("Warning: end of input while reading answer"));
    assert!(output.contains("Using default value for answer"));
}

#[test]
fn input_from_assigns_existing_variables() {
    let mut reader = Cursor::new(b"12.5\n2.5\n");
    let mut writer = Vec::new();

    let distance: f64;
    let time: f64;

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        distance,
        time,
    }

    assert_eq!(distance, 12.5);
    assert_eq!(time, 2.5);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = time = ");
}

#[test]
fn input_from_declares_typed_variables() {
    let mut reader = Cursor::new(b"12.5\n2.5\nTimothy\n");
    let mut writer = Vec::new();

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        distance: f64,
        time: f64,
        name: String,
    }

    assert_eq!(distance, 12.5);
    assert_eq!(time, 2.5);
    assert_eq!(name, "Timothy");

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = time = name = ");
}

#[test]
fn input_from_declares_typed_variables_and_defaults_missing_input() {
    let mut reader = Cursor::new(b"12.5\n");
    let mut writer = Vec::new();

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        distance: f64,
        time: f64,
        name: String,
    }

    assert_eq!(distance, 12.5);
    assert_eq!(time, 0.0);
    assert_eq!(name, "");

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert!(output.contains("distance = "));
    assert!(output.contains("time = "));
    assert!(output.contains("name = "));
    assert!(output.contains("Warning: end of input while reading time"));
    assert!(output.contains("Warning: end of input while reading name"));
}

#[test]
fn output_to_writes_comma_separated_template_lines() {
    let distance = 10.0;
    let velocity = 5.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        distance = {distance},
        velocity = {velocity},
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = 10\nvelocity = 5\n");
}

#[test]
fn output_to_writes_text_without_string_quotes() {
    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        Hello world,
        This is plain output text,
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "Hello world\nThis is plain output text\n");
}

#[test]
fn output_to_interpolates_expressions_inside_braces() {
    let distance = 12.5;
    let time = 2.5;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        Distance is {distance},
        Time is {time},
        Speed is {distance / time},
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        concat!("Distance is 12.5\n", "Time is 2.5\n", "Speed is 5\n",)
    );
}

#[test]
fn output_to_each_comma_separated_item_gets_its_own_newline() {
    let centimeters = 1000.0;
    let meters = 10.0;
    let seconds = 2.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        centimeters = {centimeters},
        meters = {meters},
        time = {seconds},
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        concat!("centimeters = 1000\n", "meters = 10\n", "time = 2\n",)
    );
}
