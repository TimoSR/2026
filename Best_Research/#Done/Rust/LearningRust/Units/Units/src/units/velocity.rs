use std::fmt;

use crate::internal::{
    QuantityError, check_nonzero, format_unit_value, implement_quantity_arithmetic,
};
use crate::{Acceleration, Time};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Velocity(pub(crate) f64);

impl Velocity {
    #[must_use]
    pub const fn meters_per_second(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn kilometers_per_hour(value: f64) -> Self {
        Self(value / 3.6)
    }

    #[must_use]
    pub const fn as_meters_per_second(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_kilometers_per_hour(self) -> f64 {
        self.0 * 3.6
    }

    #[must_use]
    pub const fn display_as(self, unit: VelocityUnit) -> VelocityDisplay {
        VelocityDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(
        self,
        unit: VelocityUnit,
        precision: usize,
    ) -> VelocityDisplay {
        VelocityDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub fn checked_div_time(self, time: Time) -> Result<Acceleration, QuantityError> {
        check_nonzero(time.as_seconds(), "Velocity / Time")?;
        Ok(self / time)
    }
}

#[must_use]
pub const fn meters_per_second(value: f64) -> Velocity {
    Velocity::meters_per_second(value)
}

#[must_use]
pub const fn kilometers_per_hour(value: f64) -> Velocity {
    Velocity::kilometers_per_hour(value)
}

implement_quantity_arithmetic!(Velocity);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VelocityUnit {
    MetersPerSecond,
    KilometersPerHour,
}

impl VelocityUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::MetersPerSecond => "m/s",
            Self::KilometersPerHour => "km/h",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VelocityDisplay {
    value: Velocity,
    unit: VelocityUnit,
    precision: Option<usize>,
}

impl fmt::Display for VelocityDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            VelocityUnit::MetersPerSecond => self.value.as_meters_per_second(),
            VelocityUnit::KilometersPerHour => self.value.as_kilometers_per_hour(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(VelocityUnit::MetersPerSecond)
            .fmt(formatter)
    }
}
