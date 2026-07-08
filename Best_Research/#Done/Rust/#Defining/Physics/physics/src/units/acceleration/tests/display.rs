use super::super::*;

#[test]
fn default_display_uses_meters_per_second_squared() {
    let acceleration = meters_per_second_squared(2.5);

    assert_eq!(acceleration.to_string(), "2.5 m/s^2");
}

#[test]
fn display_can_format_standard_gravity_with_precision() {
    let acceleration = standard_gravity(1.0);

    assert_eq!(acceleration.display_standard_gravity_precision(2).to_string(), "1.00 g0");
}
