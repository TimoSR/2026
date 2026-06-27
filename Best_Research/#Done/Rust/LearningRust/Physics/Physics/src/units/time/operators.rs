use std::ops::Mul;

use crate::{acceleration::Acceleration, internal::implement_quantity_arithmetic, length::Length, velocity::Velocity};

use super::quantity::Time;

implement_quantity_arithmetic!(Time);

impl Mul<Velocity> for Time
{
    type Output = Length;

    fn mul(self, velocity: Velocity) -> Self::Output
    {
        Length::meters(self.to_seconds() * velocity.to_meters_per_second())
    }
}

impl Mul<Acceleration> for Time
{
    type Output = Velocity;

    fn mul(self, acceleration: Acceleration) -> Self::Output
    {
        Velocity::meters_per_second(self.to_seconds() * acceleration.to_meters_per_second_squared())
    }
}
