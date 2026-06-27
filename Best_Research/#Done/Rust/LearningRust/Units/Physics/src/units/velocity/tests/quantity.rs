use super::super::*;

#[test]
fn constructors_convert_to_meters_per_second()
{
    assert_eq!(meters_per_second(12.0).to_meters_per_second(), 12.0);
    super::assert_close(kilometers_per_hour(36.0).to_meters_per_second(), 10.0);
}

#[test]
fn unit_symbols_match_velocity_units()
{
    assert_eq!(VelocityUnit::MetersPerSecond.symbol(), "m/s");
    assert_eq!(VelocityUnit::KilometersPerHour.symbol(), "km/h");
}
