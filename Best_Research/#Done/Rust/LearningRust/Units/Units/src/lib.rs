mod internal;
mod units;

pub use internal::QuantityError;
pub use units::*;

#[must_use]
pub const fn velocity(distance: Length, time: Time) -> Velocity
{
    units::velocity::calculate(distance, time)
}

pub fn checked_velocity(distance: Length, time: Time) -> Result<Velocity, QuantityError>
{
    units::velocity::checked_calculate(distance, time)
}

#[must_use]
pub const fn acceleration(velocity: Velocity, time: Time) -> Acceleration
{
    units::acceleration::calculate(velocity, time)
}

pub fn checked_acceleration(
    velocity: Velocity,
    time: Time,
) -> Result<Acceleration, QuantityError>
{
    units::acceleration::checked_calculate(velocity, time)
}

#[must_use]
pub const fn force(mass: Mass, acceleration: Acceleration) -> Force
{
    units::force::calculate(mass, acceleration)
}
