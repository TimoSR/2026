use super::super::*;

#[test]
fn standard_gravity_converts_to_meters_per_second_squared()
{
    let acceleration = standard_gravity(1.0);

    super::assert_close(acceleration.to_meters_per_second_squared(), 9.80665);
    super::assert_close(acceleration.to_standard_gravity(), 1.0);
}

#[test]
fn unit_symbols_match_acceleration_units()
{
    assert_eq!(AccelerationUnit::MetersPerSecondSquared.symbol(), "m/s^2");
    assert_eq!(AccelerationUnit::StandardGravity.symbol(), "g0");
}
