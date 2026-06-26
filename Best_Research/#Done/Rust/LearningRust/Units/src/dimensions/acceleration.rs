use std::fmt;

use crate::display::format_unit_value;
use crate::macros::implement_quantity_arithmetic;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Acceleration(pub(crate) f64);

impl Acceleration {
    pub const STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED: f64 = 9.80665;

    #[must_use]
    pub const fn meters_per_second_squared(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn standard_gravity(value: f64) -> Self {
        Self(value * Self::STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED)
    }

    #[must_use]
    pub const fn as_meters_per_second_squared(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_standard_gravity(self) -> f64 {
        self.0 / Self::STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED
    }

    #[must_use]
    pub const fn display_as(self, unit: AccelerationUnit) -> AccelerationDisplay {
        AccelerationDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(
        self,
        unit: AccelerationUnit,
        precision: usize,
    ) -> AccelerationDisplay {
        AccelerationDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }
}

implement_quantity_arithmetic!(Acceleration);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AccelerationUnit {
    MetersPerSecondSquared,
    StandardGravity,
}

impl AccelerationUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::MetersPerSecondSquared => "m/s^2",
            Self::StandardGravity => "g0",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AccelerationDisplay {
    value: Acceleration,
    unit: AccelerationUnit,
    precision: Option<usize>,
}

impl fmt::Display for AccelerationDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            AccelerationUnit::MetersPerSecondSquared => self.value.as_meters_per_second_squared(),
            AccelerationUnit::StandardGravity => self.value.as_standard_gravity(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(AccelerationUnit::MetersPerSecondSquared)
            .fmt(formatter)
    }
}
