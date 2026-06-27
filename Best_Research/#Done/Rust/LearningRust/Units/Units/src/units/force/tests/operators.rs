use crate::{Acceleration, Mass};

use super::super::*;

#[test]
fn force_divided_by_mass_returns_acceleration() {
    let acceleration = newtons(12.0) / Mass::kilograms(3.0);

    super::assert_close(acceleration.as_meters_per_second_squared(), 4.0);
}

#[test]
fn force_divided_by_acceleration_returns_mass() {
    let mass = newtons(12.0) / Acceleration::meters_per_second_squared(3.0);

    super::assert_close(mass.as_kilograms(), 4.0);
}

#[test]
fn checked_div_mass_rejects_zero_mass() {
    let result = newtons(12.0).checked_div_mass(Mass::kilograms(0.0));

    assert!(result.is_err());
}
