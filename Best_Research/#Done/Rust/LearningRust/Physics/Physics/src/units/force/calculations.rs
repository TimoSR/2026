use crate::{Acceleration, Mass};

use super::quantity::Force;

 
pub const fn calculate(mass: Mass, acceleration: Acceleration) -> Force
{
    Force::newtons(mass.to_kilograms() * acceleration.to_meters_per_second_squared())
}
