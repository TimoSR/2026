use super::super::*;

#[test]
fn default_display_uses_meters() {
    let length = meters(12.5);

    assert_eq!(length.to_string(), "12.5 m");
}

#[test]
fn display_can_format_kilometers_with_precision() {
    let length = meters(1_250.0);

    assert_eq!(
        length
            .display_as_precision(LengthUnit::Kilometers, 2)
            .to_string(),
        "1.25 km"
    );
}
