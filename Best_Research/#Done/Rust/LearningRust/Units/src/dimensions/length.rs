use std::fmt;

use crate::display::format_unit_value;
use crate::error::{check_nonzero, validate_finite, QuantityError};
use crate::macros::implement_quantity_arithmetic;
use crate::{Time, Velocity};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Length(pub(crate) f64);

impl Length {
    #[must_use]
    pub const fn meters(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn kilometers(value: f64) -> Self {
        Self(value * 1_000.0)
    }

    #[must_use]
    pub const fn centimeters(value: f64) -> Self {
        Self(value / 100.0)
    }

    #[must_use]
    pub const fn millimeters(value: f64) -> Self {
        Self(value / 1_000.0)
    }

    #[must_use]
    pub const fn micrometers(value: f64) -> Self {
        Self(value / 1_000_000.0)
    }

    #[must_use]
    pub const fn nanometers(value: f64) -> Self {
        Self(value / 1_000_000_000.0)
    }

    pub fn try_meters(value: f64) -> Result<Self, QuantityError> {
        Ok(Self(validate_finite("Length", "m", value)?))
    }

    pub fn try_centimeters(value: f64) -> Result<Self, QuantityError> {
        Ok(Self(validate_finite("Length", "cm", value)? / 100.0))
    }

    #[must_use]
    pub const fn as_meters(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_kilometers(self) -> f64 {
        self.0 / 1_000.0
    }

    #[must_use]
    pub const fn as_centimeters(self) -> f64 {
        self.0 * 100.0
    }

    #[must_use]
    pub const fn as_millimeters(self) -> f64 {
        self.0 * 1_000.0
    }

    #[must_use]
    pub const fn as_micrometers(self) -> f64 {
        self.0 * 1_000_000.0
    }

    #[must_use]
    pub const fn as_nanometers(self) -> f64 {
        self.0 * 1_000_000_000.0
    }

    #[must_use]
    pub const fn display_as(self, unit: LengthUnit) -> LengthDisplay {
        LengthDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(self, unit: LengthUnit, precision: usize) -> LengthDisplay {
        LengthDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub fn checked_div_time(self, time: Time) -> Result<Velocity, QuantityError> {
        check_nonzero(time.as_seconds(), "Length / Time")?;
        Ok(self / time)
    }
}

implement_quantity_arithmetic!(Length);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LengthUnit {
    Meters,
    Kilometers,
    Centimeters,
    Millimeters,
    Micrometers,
    Nanometers,
}

impl LengthUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Meters => "m",
            Self::Kilometers => "km",
            Self::Centimeters => "cm",
            Self::Millimeters => "mm",
            Self::Micrometers => "um",
            Self::Nanometers => "nm",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LengthDisplay {
    value: Length,
    unit: LengthUnit,
    precision: Option<usize>,
}

impl fmt::Display for LengthDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            LengthUnit::Meters => self.value.as_meters(),
            LengthUnit::Kilometers => self.value.as_kilometers(),
            LengthUnit::Centimeters => self.value.as_centimeters(),
            LengthUnit::Millimeters => self.value.as_millimeters(),
            LengthUnit::Micrometers => self.value.as_micrometers(),
            LengthUnit::Nanometers => self.value.as_nanometers(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Length {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(LengthUnit::Meters).fmt(formatter)
    }
}
