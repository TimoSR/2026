use std::ops::Div;

use crate::internal::{QuantityError, check_nonzero, implement_quantity_arithmetic};
use crate::{Time, Velocity};

use super::quantity::Length;

implement_quantity_arithmetic!(Length);

impl Length {
    pub fn checked_div_time(self, time: Time) -> Result<Velocity, QuantityError> {
        check_nonzero(time.to_seconds(), "Length / Time")?;
        Ok(self / time)
    }
}

impl Div<Time> for Length {
    type Output = Velocity;

    fn div(self, time: Time) -> Self::Output {
        Velocity::meters_per_second(self.to_meters() / time.to_seconds())
    }
}

impl Div<Velocity> for Length {
    type Output = Time;

    fn div(self, velocity: Velocity) -> Self::Output {
        Time::seconds(self.to_meters() / velocity.to_meters_per_second())
    }
}
