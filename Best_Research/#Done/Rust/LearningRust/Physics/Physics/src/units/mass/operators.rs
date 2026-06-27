use std::ops::Mul;

use crate::{acceleration::Acceleration, force::Force, internal::implement_quantity_arithmetic};

use super::quantity::Mass;

implement_quantity_arithmetic!(Mass);

impl Mul<Acceleration> for Mass
{
    type Output = Force;

    fn mul(self, acceleration: Acceleration) -> Self::Output
    {
        Force::newtons(self.to_kilograms() * acceleration.to_meters_per_second_squared())
    }
}
