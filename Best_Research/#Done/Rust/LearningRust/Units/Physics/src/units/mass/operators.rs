use std::ops::Mul;

use crate::internal::implement_quantity_arithmetic;
use crate::{Acceleration, Force};

use super::quantity::Mass;

implement_quantity_arithmetic!(Mass);

impl Mul<Acceleration> for Mass {
    type Output = Force;

    fn mul(self, acceleration: Acceleration) -> Self::Output {
        Force::newtons(self.to_kilograms() * acceleration.to_meters_per_second_squared())
    }
}
