use crate::{Mass, Time, Velocity};

use super::super::*;

#[test]
fn acceleration_multiplied_by_time_returns_velocity()
{
    let velocity = meters_per_second_squared(3.0) * Time::seconds(4.0);

    super::assert_close(velocity.to_meters_per_second(), 12.0);
}

#[test]
fn calculate_returns_velocity_divided_by_time()
{
    let acceleration = calculate(Velocity::meters_per_second(12.0), Time::seconds(4.0));

    super::assert_close(acceleration.to_meters_per_second_squared(), 3.0);
}

#[test]
fn acceleration_multiplied_by_mass_returns_force()
{
    let force = meters_per_second_squared(3.0) * Mass::kilograms(4.0);

    super::assert_close(force.to_newtons(), 12.0);
}

#[test]
fn same_dimension_arithmetic_is_available()
{
    let acceleration = meters_per_second_squared(2.0) + meters_per_second_squared(3.0);

    assert_eq!(acceleration, meters_per_second_squared(5.0));
    assert!(acceleration.approximately_equals(meters_per_second_squared(5.0000001), 0.000001));
}

#[test]
fn checked_calculate_rejects_zero_time()
{
    let result = checked_calculate(Velocity::meters_per_second(12.0), Time::seconds(0.0));

    assert!(result.is_err());
}
