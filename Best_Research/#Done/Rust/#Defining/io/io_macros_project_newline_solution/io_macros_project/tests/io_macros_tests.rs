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

    let mut distance = 0.0_f64;
    let mut time = 0.0_f64;

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
fn output_to_respects_source_newlines() {
    let distance = 10.0;
    let centimeters = 1000.0;
    let meters = 10.0;
    let seconds = 2.0;
    let kilograms = 4.0;
    let kilometers_per_hour = 18.0;
    let acceleration = 2.5;
    let gravity = 0.25492900154487036;
    let kilonewtons = 0.01;
    let hello = "Hello";

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        distance = {distance} bubels = {centimeters} centimeters
        centimeters = {centimeters}
        meters = {meters}
        time = {seconds} seconds
        mass = {kilograms} kilograms
        velocity = {kilometers_per_hour} km/h
        acceleration = {acceleration} m/s²
        gravity = {gravity} g
        force = {kilonewtons} kN
        force = {hello}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        concat!(
            "distance = 10 bubels = 1000 centimeters\n",
            "centimeters = 1000\n",
            "meters = 10\n",
            "time = 2 seconds\n",
            "mass = 4 kilograms\n",
            "velocity = 18 km/h\n",
            "acceleration = 2.5 m/s²\n",
            "gravity = 0.25492900154487036 g\n",
            "force = 0.01 kN\n",
            "force = Hello\n",
        )
    );
}

#[test]
fn output_to_supports_plain_text_and_multiple_interpolations() {
    let name = "Timothy";
    let distance = 10.0;
    let centimeters = 1000.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        Hello {name}
        distance = {distance} bubels = {centimeters} centimeters
        Done
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        concat!(
            "Hello Timothy\n",
            "distance = 10 bubels = 1000 centimeters\n",
            "Done\n",
        )
    );
}
