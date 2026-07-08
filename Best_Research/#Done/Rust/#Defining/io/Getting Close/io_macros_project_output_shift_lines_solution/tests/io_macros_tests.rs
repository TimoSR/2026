#![recursion_limit = "1024"]

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
fn input_from_supports_let_and_let_mut_typed_declarations() {
    let mut reader = Cursor::new(b"12.5\n2.5\n4.0\n");
    let mut writer = Vec::new();

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        let distance: f64,
        let time: f64,
        let mut mass: f64,
    }

    assert_eq!(distance, 12.5);
    assert_eq!(time, 2.5);
    assert_eq!(mass, 4.0);

    mass += 1.0;

    assert_eq!(mass, 5.0);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = time = mass = ");
}

#[test]
fn input_from_mixes_let_declarations_and_existing_variables() {
    let mut reader = Cursor::new(b"12.5\n2.5\n4.0\n");
    let mut writer = Vec::new();

    let mut time = 0.0_f64;
    let mut mass = 0.0_f64;

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        let distance: f64,
        time,
        mass,
    }

    assert_eq!(distance, 12.5);
    assert_eq!(time, 2.5);
    assert_eq!(mass, 4.0);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = time = mass = ");
}

#[test]
fn output_to_uses_shift_markers_as_line_starts() {
    let distance = 10.0;
    let centimeters = 1000.0;
    let meters = 10.0;
    let hello = "Hello";

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << [measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.
        << [summary: force] message [{hello}].
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        concat!(
            "[measurement: distance] raw input = 10, converted value = 1000 centimeters, normalized value = 10 meters.\n",
            "[summary: force] message [Hello].\n",
        )
    );
}

#[test]
fn output_to_allows_commas_periods_colons_brackets_and_semicolons() {
    let seconds = 2.0;
    let time = 2.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        "[time details] time = 2 seconds, source value = 2, status: accepted;\n"
    );
}

#[test]
fn output_to_handles_long_complex_text_with_nested_bracket_interpolations() {
    let distance = 10.0;
    let time = 2.0;
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
        << [measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.
        << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
        << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
        << [mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.
        << [motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.
        << [motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.
        << [gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.
        << [force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.
        << [summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].
        << [summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].
        << [summary: force] force [{kilonewtons} kN], message [{hello}].
        << [report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.
        << [report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.
        << [report] Compared with Earth gravity, this acceleration is {gravity} g.
        << [report] Final force output: {kilonewtons} kN.
        << [message] force label reused with text value: {hello}.
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert!(output.contains("[measurement: distance] raw input = 10, converted value = 1000 centimeters, normalized value = 10 meters."));
    assert!(output.contains("[summary: values] distance [10], centimeters [1000], meters [10], seconds [2], kilograms [4]."));
    assert!(output.contains("[summary: motion] velocity [18 km/h], acceleration [2.5 m/s^2], gravity [0.25492900154487036 g]."));
    assert!(output.contains("[summary: force] force [0.01 kN], message [Hello]."));
    assert!(output.contains("[time details] time = 2 seconds, source value = 2, status: accepted;"));
}
