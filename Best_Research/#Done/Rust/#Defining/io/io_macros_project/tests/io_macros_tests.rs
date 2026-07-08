use std::io::Cursor;

use io_macros_project::{input_from, output_to, read_value_from};

#[test]
fn read_value_retries_until_valid_input() {
    let mut reader = Cursor::new(b"not-a-number\n42\n");
    let mut writer = Vec::new();

    let value: i32 = read_value_from(&mut reader, &mut writer, "answer");

    assert_eq!(value, 42);

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");
    assert!(output.contains("answer = "));
    assert!(output.contains("Invalid answer"));
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
}

#[test]
fn output_to_writes_labeled_values() {
    let distance = 10.0;
    let velocity = 5.0;

    let mut writer = Vec::new();

    output_to! {
        writer: &mut writer,
        distance = {distance}
        velocity = {velocity}
    }

    let output = String::from_utf8(writer).expect("writer should contain valid UTF-8");

    assert_eq!(
        output,
        "distance = 10\nvelocity = 5\n"
    );
}
