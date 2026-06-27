use crate::internal::{QuantityError, check_nonzero};
use crate::{Time, Velocity};

use super::quantity::Acceleration;

#[must_use]
pub const fn calculate(velocity: Velocity, time: Time) -> Acceleration
{
    Acceleration::meters_per_second_squared(velocity.as_meters_per_second() / time.as_seconds())
}

pub fn checked_calculate(
    velocity: Velocity,
    time: Time,
) -> Result<Acceleration, QuantityError>
{
    check_nonzero(time.as_seconds(), "acceleration::calculate")?;
    Ok(calculate(velocity, time))
}
