use crate::{Acceleration, Velocity};

use super::super::*;

#[test]
fn time_multiplied_by_velocity_returns_length()
{
    let length = seconds(4.0) * Velocity::meters_per_second(3.0);

    super::assert_close(length.to_meters(), 12.0);
}

#[test]
fn time_multiplied_by_acceleration_returns_velocity()
{
    let velocity = seconds(4.0) * Acceleration::meters_per_second_squared(3.0);

    super::assert_close(velocity.to_meters_per_second(), 12.0);
}

#[test]
fn same_dimension_arithmetic_is_available()
{
    let time = seconds(2.0) + seconds(3.0);

    assert_eq!(time, seconds(5.0));
    assert!(time.approximately_equals(seconds(5.0000001), 0.000001));
}
