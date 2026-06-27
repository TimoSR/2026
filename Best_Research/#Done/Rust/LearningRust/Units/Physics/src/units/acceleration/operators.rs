use std::ops::Mul;

use crate::internal::implement_quantity_arithmetic;
use crate::{Force, Mass, Time, Velocity};

use super::quantity::Acceleration;

implement_quantity_arithmetic!(Acceleration);

impl Mul<Time> for Acceleration {
    type Output = Velocity;

    fn mul(self, time: Time) -> Self::Output {
        Velocity::meters_per_second(self.to_meters_per_second_squared() * time.to_seconds())
    }
}

impl Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, mass: Mass) -> Self::Output {
        Force::newtons(self.to_meters_per_second_squared() * mass.to_kilograms())
    }
}
