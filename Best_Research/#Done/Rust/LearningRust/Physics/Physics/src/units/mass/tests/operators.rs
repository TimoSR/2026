use crate::Acceleration;

use super::super::*;

#[test]
fn mass_multiplied_by_acceleration_returns_force()
{
    let force = kilograms(4.0) * Acceleration::meters_per_second_squared(3.0);

    super::assert_close(force.to_newtons(), 12.0);
}

#[test]
fn same_dimension_arithmetic_is_available()
{
    let mass = kilograms(2.0) + kilograms(3.0);

    assert_eq!(mass, kilograms(5.0));
    assert!(mass.approximately_equals(kilograms(5.0000001), 0.000001));
}
