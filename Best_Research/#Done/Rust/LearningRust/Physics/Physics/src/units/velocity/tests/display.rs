use super::super::*;

#[test]
fn default_display_uses_meters_per_second() {
    let velocity = meters_per_second(12.5);

    assert_eq!(velocity.to_string(), "12.5 m/s");
}

#[test]
fn display_can_format_kilometers_per_hour_with_precision() {
    let velocity = meters_per_second(10.0);

    assert_eq!(velocity.display_kilometers_per_hour_precision(2).to_string(), "36.00 km/h");
}
