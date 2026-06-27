mod display;
mod operators;
mod quantity;

#[cfg(test)]
mod tests;

pub use display::VelocityDisplay;
pub use quantity::{Velocity, VelocityUnit, kilometers_per_hour, meters_per_second};
