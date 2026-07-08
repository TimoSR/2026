use std::ops::Mul;

use crate::acceleration::Acceleration;
use crate::force::Force;
use crate::implement_quantity_arithmetic;
use crate::mass::Mass;
use crate::time::Time;
use crate::velocity::Velocity;

implement_quantity_arithmetic!(Acceleration);

impl Mul<Time> for Acceleration {
    type Output = Velocity;

    fn mul(self, time: Time) -> Self::Output { Velocity::meters_per_second(self.to_meters_per_second_squared() * time.to_seconds()) }
}

impl Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, mass: Mass) -> Self::Output { Force::newtons(self.to_meters_per_second_squared() * mass.to_kilograms()) }
}
