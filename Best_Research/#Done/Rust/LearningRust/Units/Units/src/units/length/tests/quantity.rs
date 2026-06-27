use super::super::*;

#[test]
fn constructors_convert_to_meters() {
    assert_eq!(meters(12.0).as_meters(), 12.0);
    assert_eq!(kilometers(1.5).as_meters(), 1_500.0);
    assert_eq!(centimeters(250.0).as_meters(), 2.5);
    assert_eq!(millimeters(1_200.0).as_meters(), 1.2);
}

#[test]
fn fallible_constructors_reject_non_finite_values() {
    assert!(Length::try_meters(f64::INFINITY).is_err());
    assert!(Length::try_centimeters(f64::NAN).is_err());
}

#[test]
fn unit_symbols_match_length_units() {
    assert_eq!(LengthUnit::Meters.symbol(), "m");
    assert_eq!(LengthUnit::Kilometers.symbol(), "km");
    assert_eq!(LengthUnit::Centimeters.symbol(), "cm");
    assert_eq!(LengthUnit::Millimeters.symbol(), "mm");
    assert_eq!(LengthUnit::Micrometers.symbol(), "um");
    assert_eq!(LengthUnit::Nanometers.symbol(), "nm");
}
