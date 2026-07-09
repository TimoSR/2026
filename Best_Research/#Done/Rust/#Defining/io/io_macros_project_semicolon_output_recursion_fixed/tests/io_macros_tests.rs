use std::fmt;
use std::io::Cursor;

use io_macros_project::{InputError, input_from, output_to, read_value_from, try_read_value_from};

struct CustomDisplayValue(u32);

impl fmt::Display for CustomDisplayValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "custom-{}", self.0)
    }
}

#[test]
fn try_read_value_retries_until_valid_input() {
    let mut reader = Cursor::new(b"not-a-number\n42\n");
    let mut writer = Vec::new();

    let value: i32 = try_read_value_from(&mut reader, &mut writer, "answer").expect("valid fallback input should parse");

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
fn input_from_declares_typed_variables_with_plain_name_colon_type() {
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
fn input_from_supports_let_without_type_when_type_can_be_inferred() {
    let mut reader = Cursor::new(b"12.5\n4.0\n");
    let mut writer = Vec::new();

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        let distance: f64,
        let mut mass,
    }

    let mass_before_mutation: f64 = mass;

    assert_eq!(mass_before_mutation, 4.0_f64);

    mass += 1.0_f64;

    assert_eq!(distance, 12.5);
    assert_eq!(mass, 5.0);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = mass = ");
}

#[test]
fn input_from_mixes_typed_declarations_and_existing_variables() {
    let mut reader = Cursor::new(b"12.5\n2.5\n4.0\n3.0\n");
    let mut writer = Vec::new();

    let time: f64;
    let mass: f64;

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        let distance: f64,
        time,
        let mut bonus_mass: f64,
        mass,
    }

    bonus_mass += 1.0;

    assert_eq!(distance, 12.5);
    assert_eq!(time, 2.5);
    assert_eq!(bonus_mass, 5.0);
    assert_eq!(mass, 3.0);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = time = bonus_mass = mass = ");
}

#[test]
fn input_from_mixed_items_can_appear_in_any_order() {
    let mut reader = Cursor::new(b"2.5\n12.5\n4.0\nTimothy\n");
    let mut writer = Vec::new();

    let time: f64;
    let name: String;

    input_from! {
        reader: &mut reader,
        writer: &mut writer,
        time,
        let distance: f64,
        mass: f64,
        name,
    }

    assert_eq!(time, 2.5);
    assert_eq!(distance, 12.5);
    assert_eq!(mass, 4.0);
    assert_eq!(name, "Timothy");

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "time = distance = mass = name = ");
}

#[test]
fn output_to_writes_arrow_marked_template_lines() {
    let distance = 10.0;
    let velocity = 5.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << distance = {distance}
        << velocity = {velocity}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = 10\nvelocity = 5\n");
}

#[test]
fn output_to_interpolates_expressions_inside_braces() {
    let distance = 12.5;
    let time = 2.5;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << Distance is {distance}
        << Time is {time}
        << Speed is {distance / time}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, concat!("Distance is 12.5\n", "Time is 2.5\n", "Speed is 5\n",));
}

#[test]
fn output_to_supports_commas_inside_template_lines() {
    let distance = 10.0;
    let centimeters = 1000.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << distance = {distance}, bubels = {centimeters} centimeters
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "distance = 10, bubels = 1000 centimeters\n");
}

#[test]
fn output_to_respects_spacing_around_values_units_and_punctuation() {
    let distance = 10.0;
    let centimeters = 1000.0;
    let meters = 10.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
        << Please give me input!
        << Ready?
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, concat!("[distance details] distance = 10, bubels = 1000 centimeters, meters = 10.\n", "Please give me input!\n", "Ready?\n",));
}

#[test]
fn output_to_supports_string_slices_without_string_from() {
    let hello = "Hello";
    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << force = {hello}
        << greeting = {hello}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, concat!("force = Hello\n", "greeting = Hello\n",));
}

#[test]
fn output_to_still_supports_owned_strings() {
    let hello = String::from("Hello");
    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << force = {hello}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, "force = Hello\n");
}

#[test]
fn output_to_formats_arrays_vectors_and_slices_without_debug_marker() {
    let array = [1, 3, 4];
    let vector = vec![2, 4, 8];
    let slice = &vector[1..];
    let words = ["red", "blue"];
    let hello = String::from("Hello");
    let custom = CustomDisplayValue(7);
    let custom_list = [CustomDisplayValue(1), CustomDisplayValue(2)];

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        << array = {array}
        << vector = {vector}
        << slice = {slice}
        << words = {words}
        << string still displays without quotes = {hello}
        << custom display = {custom}
        << custom display list = {custom_list}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(output, concat!("array = [1, 3, 4]\n", "vector = [2, 4, 8]\n", "slice = [4, 8]\n", "words = [red, blue]\n", "string still displays without quotes = Hello\n", "custom display = custom-7\n", "custom display list = [custom-1, custom-2]\n",));
}

#[test]
fn output_to_handles_long_complex_text_without_recursion_limit_failure() {
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
    assert!(output.contains("[time details] time = 2 seconds, source value = 2, status: accepted;"));
    assert!(output.contains("[summary: force] force [0.01 kN], message [Hello]."));
    assert!(output.contains("[message] force label reused with text value: Hello."));
}
