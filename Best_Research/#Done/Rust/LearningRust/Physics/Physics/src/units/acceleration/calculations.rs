use crate::{QuantityError, acceleration::Acceleration, check_nonzero, time::Time, velocity::Velocity};

pub const fn calculate(velocity: Velocity, time: Time) -> Acceleration
{
    Acceleration::meters_per_second_squared(velocity.to_meters_per_second() / time.to_seconds())
}

pub fn checked_calculate(velocity: Velocity, time: Time) -> Result<Acceleration, QuantityError>
{
    check_nonzero(time.to_seconds(), "acceleration::calculate")?;
    Ok(calculate(velocity, time))
}
