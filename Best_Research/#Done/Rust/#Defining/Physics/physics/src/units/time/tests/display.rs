use super::super::*;

#[test]
fn default_display_uses_seconds() {
    let time = seconds(12.5);

    assert_eq!(time.to_string(), "12.5 s");
}

#[test]
fn display_can_format_minutes_with_precision() {
    let time = seconds(90.0);

    assert_eq!(time.display_minutes_precision(2).to_string(), "1.50 min");
}
