use std::ops::Div;
use std::ops::Mul;

use crate::acceleration::Acceleration;
use crate::internal::QuantityError;
use crate::internal::check_nonzero;
use crate::internal::implement_quantity_arithmetic;
use crate::length::Length;
use crate::time::Time;

use super::quantity::Velocity;

implement_quantity_arithmetic!(Velocity);

impl Velocity {
    pub fn checked_div_time(self, time: Time) -> Result<Acceleration, QuantityError> {
        check_nonzero(time.to_seconds(), "Velocity / Time")?;
        Ok(self / time)
    }
}

impl Mul<Time> for Velocity {
    type Output = Length;

    fn mul(self, time: Time) -> Self::Output { Length::meters(self.to_meters_per_second() * time.to_seconds()) }
}

impl Div<Time> for Velocity {
    type Output = Acceleration;

    fn div(self, time: Time) -> Self::Output { Acceleration::meters_per_second_squared(self.to_meters_per_second() / time.to_seconds()) }
}

impl Div<Acceleration> for Velocity {
    type Output = Time;

    fn div(self, acceleration: Acceleration) -> Self::Output {
        Time::seconds(self.to_meters_per_second() / acceleration.to_meters_per_second_squared())
    }
}
