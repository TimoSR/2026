use super::super::*;

#[test]
fn default_display_uses_meters()
{
    let length = meters(12.5);

    assert_eq!(length.to_string(), "12.5 m");
}

#[test]
fn display_meters_formats_with_meter_symbol()
{
    let length = centimeters(10_000.0);

    assert_eq!(length.display_meters().to_string(), "100 m");
}

#[test]
fn display_centimeters_formats_with_centimeter_symbol()
{
    let length = meters(100.0);

    assert_eq!(length.display_centimeters().to_string(), "10000 cm");
}

#[test]
fn display_can_format_kilometers_with_precision()
{
    let length = meters(1_250.0);

    assert_eq!(length.display_kilometers_precision(2).to_string(), "1.25 km");
}
