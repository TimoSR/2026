mod dimensions;
mod display;
mod error;
mod macros;
mod rules;

pub use dimensions::{
    Acceleration, AccelerationUnit, Force, ForceUnit, Length, LengthUnit, Mass, MassUnit, Time,
    TimeUnit, Velocity, VelocityUnit,
};
pub use error::QuantityError;

pub mod length {
    pub use crate::dimensions::{Length, LengthUnit};

    #[must_use]
    pub const fn meters(value: f64) -> Length {
        Length::meters(value)
    }

    #[must_use]
    pub const fn kilometers(value: f64) -> Length {
        Length::kilometers(value)
    }

    #[must_use]
    pub const fn centimeters(value: f64) -> Length {
        Length::centimeters(value)
    }

    #[must_use]
    pub const fn millimeters(value: f64) -> Length {
        Length::millimeters(value)
    }

    #[must_use]
    pub const fn micrometers(value: f64) -> Length {
        Length::micrometers(value)
    }

    #[must_use]
    pub const fn nanometers(value: f64) -> Length {
        Length::nanometers(value)
    }
}

pub mod time {
    pub use crate::dimensions::{Time, TimeUnit};

    #[must_use]
    pub const fn seconds(value: f64) -> Time {
        Time::seconds(value)
    }

    #[must_use]
    pub const fn milliseconds(value: f64) -> Time {
        Time::milliseconds(value)
    }

    #[must_use]
    pub const fn microseconds(value: f64) -> Time {
        Time::microseconds(value)
    }

    #[must_use]
    pub const fn nanoseconds(value: f64) -> Time {
        Time::nanoseconds(value)
    }

    #[must_use]
    pub const fn minutes(value: f64) -> Time {
        Time::minutes(value)
    }

    #[must_use]
    pub const fn hours(value: f64) -> Time {
        Time::hours(value)
    }
}

pub mod mass {
    pub use crate::dimensions::{Mass, MassUnit};

    #[must_use]
    pub const fn kilograms(value: f64) -> Mass {
        Mass::kilograms(value)
    }

    #[must_use]
    pub const fn kilogram(value: f64) -> Mass {
        Mass::kilogram(value)
    }

    #[must_use]
    pub const fn grams(value: f64) -> Mass {
        Mass::grams(value)
    }

    #[must_use]
    pub const fn milligrams(value: f64) -> Mass {
        Mass::milligrams(value)
    }

    #[must_use]
    pub const fn micrograms(value: f64) -> Mass {
        Mass::micrograms(value)
    }

    #[must_use]
    pub const fn tons(value: f64) -> Mass {
        Mass::tons(value)
    }
}

pub mod velocity {
    pub use crate::dimensions::{Velocity, VelocityUnit};

    #[must_use]
    pub const fn meters_per_second(value: f64) -> Velocity {
        Velocity::meters_per_second(value)
    }

    #[must_use]
    pub const fn meters_pr_second(value: f64) -> Velocity {
        Velocity::meters_per_second(value)
    }

    #[must_use]
    pub const fn kilometers_per_hour(value: f64) -> Velocity {
        Velocity::kilometers_per_hour(value)
    }
}

pub mod acceleration {
    pub use crate::dimensions::{Acceleration, AccelerationUnit};

    #[must_use]
    pub const fn meters_per_second_squared(value: f64) -> Acceleration {
        Acceleration::meters_per_second_squared(value)
    }

    #[must_use]
    pub const fn meters_pr_second_pr_second(value: f64) -> Acceleration {
        Acceleration::meters_per_second_squared(value)
    }

    #[must_use]
    pub const fn standard_gravity(value: f64) -> Acceleration {
        Acceleration::standard_gravity(value)
    }
}

pub mod force {
    pub use crate::dimensions::{Force, ForceUnit};

    #[must_use]
    pub const fn newtons(value: f64) -> Force {
        Force::newtons(value)
    }

    #[must_use]
    pub const fn force(value: f64) -> Force {
        Force::newtons(value)
    }

    #[must_use]
    pub const fn millinewtons(value: f64) -> Force {
        Force::millinewtons(value)
    }

    #[must_use]
    pub const fn kilonewtons(value: f64) -> Force {
        Force::kilonewtons(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_units_normalize_to_meters() {
        assert_eq!(length::meters(1.0), length::centimeters(100.0));
        assert_eq!(length::centimeters(100.0), length::millimeters(1_000.0));
        assert_eq!(length::meters(1.0), length::nanometers(1_000_000_000.0));
    }

    #[test]
    fn time_units_normalize_to_seconds() {
        assert_eq!(time::seconds(1.0), time::milliseconds(1_000.0));
        assert_eq!(time::seconds(1.0), time::microseconds(1_000_000.0));
        assert_eq!(time::seconds(1.0), time::nanoseconds(1_000_000_000.0));
        assert_eq!(time::minutes(1.0), time::seconds(60.0));
    }

    #[test]
    fn mass_units_normalize_to_kilograms() {
        assert_eq!(mass::kilogram(1.0), mass::grams(1_000.0));
        assert_eq!(mass::kilogram(1.0), mass::milligrams(1_000_000.0));
        assert_eq!(mass::kilogram(1.0), mass::tons(0.001));
    }

    #[test]
    fn equation_rules_ignore_input_scale() {
        let distance = length::centimeters(10_000.0);
        let time = time::milliseconds(9_580.0);
        let mass = mass::grams(80_000.0);

        let velocity: Velocity = distance / time;
        let acceleration: Acceleration = velocity / time;
        let force: Force = mass * acceleration;

        assert!(velocity.approximately_equals(length::meters(100.0) / time::seconds(9.58), 1e-12));
        assert!(acceleration.approximately_equals(velocity / time::seconds(9.58), 1e-12));
        assert!(force.approximately_equals(mass * acceleration, 1e-12));
    }

    #[test]
    fn checked_division_reports_zero_time() {
        let result = length::meters(10.0).checked_div_time(time::seconds(0.0));

        assert_eq!(
            result,
            Err(QuantityError::DivisionByZero {
                operation: "Length / Time",
            })
        );
    }
}
