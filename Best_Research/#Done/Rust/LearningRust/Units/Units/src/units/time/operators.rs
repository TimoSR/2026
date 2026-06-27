use std::ops::Mul;

use crate::internal::implement_quantity_arithmetic;
use crate::{Acceleration, Length, Velocity};

use super::quantity::Time;

implement_quantity_arithmetic!(Time);

impl Mul<Velocity> for Time {
    type Output = Length;

    fn mul(self, velocity: Velocity) -> Self::Output {
        Length::meters(self.as_seconds() * velocity.as_meters_per_second())
    }
}

impl Mul<Acceleration> for Time {
    type Output = Velocity;

    fn mul(self, acceleration: Acceleration) -> Self::Output {
        Velocity::meters_per_second(
            self.as_seconds() * acceleration.as_meters_per_second_squared(),
        )
    }
}
