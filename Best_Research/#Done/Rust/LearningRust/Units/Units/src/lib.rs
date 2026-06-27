mod internal;
mod units;

pub use internal::QuantityError;
pub use units::*;

pub use units::acceleration::calculate as acceleration;
pub use units::acceleration::checked_calculate as checked_acceleration;
pub use units::force::calculate as force;
pub use units::velocity::calculate as velocity;
pub use units::velocity::checked_calculate as checked_velocity;
