use crate::{Acceleration, Mass};

use super::quantity::Force;

#[must_use]
pub const fn calculate(mass: Mass, acceleration: Acceleration) -> Force
{
    Force::newtons(mass.as_kilograms() * acceleration.as_meters_per_second_squared())
}
