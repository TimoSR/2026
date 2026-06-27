use crate::internal::{QuantityError, check_nonzero};
use crate::{Length, Time};

use super::quantity::Velocity;

#[must_use]
pub const fn calculate(distance: Length, time: Time) -> Velocity
{
    Velocity::meters_per_second(distance.to_meters() / time.to_seconds())
}

pub fn checked_calculate(distance: Length, time: Time) -> Result<Velocity, QuantityError>
{
    check_nonzero(time.to_seconds(), "velocity::calculate")?;
    Ok(calculate(distance, time))
}
