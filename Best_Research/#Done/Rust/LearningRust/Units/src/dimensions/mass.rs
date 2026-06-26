use std::fmt;

use crate::display::format_unit_value;
use crate::error::{validate_finite, QuantityError};
use crate::macros::implement_quantity_arithmetic;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Mass(pub(crate) f64);

impl Mass {
    #[must_use]
    pub const fn kilograms(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn kilogram(value: f64) -> Self {
        Self::kilograms(value)
    }

    #[must_use]
    pub const fn grams(value: f64) -> Self {
        Self(value / 1_000.0)
    }

    #[must_use]
    pub const fn milligrams(value: f64) -> Self {
        Self(value / 1_000_000.0)
    }

    #[must_use]
    pub const fn micrograms(value: f64) -> Self {
        Self(value / 1_000_000_000.0)
    }

    #[must_use]
    pub const fn tons(value: f64) -> Self {
        Self(value * 1_000.0)
    }

    pub fn try_kilograms(value: f64) -> Result<Self, QuantityError> {
        Ok(Self(validate_finite("Mass", "kg", value)?))
    }

    #[must_use]
    pub const fn as_kilograms(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_grams(self) -> f64 {
        self.0 * 1_000.0
    }

    #[must_use]
    pub const fn as_milligrams(self) -> f64 {
        self.0 * 1_000_000.0
    }

    #[must_use]
    pub const fn as_micrograms(self) -> f64 {
        self.0 * 1_000_000_000.0
    }

    #[must_use]
    pub const fn as_tons(self) -> f64 {
        self.0 / 1_000.0
    }

    #[must_use]
    pub const fn display_as(self, unit: MassUnit) -> MassDisplay {
        MassDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(self, unit: MassUnit, precision: usize) -> MassDisplay {
        MassDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }
}

implement_quantity_arithmetic!(Mass);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MassUnit {
    Kilograms,
    Grams,
    Milligrams,
    Micrograms,
    Tons,
}

impl MassUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Kilograms => "kg",
            Self::Grams => "g",
            Self::Milligrams => "mg",
            Self::Micrograms => "ug",
            Self::Tons => "t",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MassDisplay {
    value: Mass,
    unit: MassUnit,
    precision: Option<usize>,
}

impl fmt::Display for MassDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            MassUnit::Kilograms => self.value.as_kilograms(),
            MassUnit::Grams => self.value.as_grams(),
            MassUnit::Milligrams => self.value.as_milligrams(),
            MassUnit::Micrograms => self.value.as_micrograms(),
            MassUnit::Tons => self.value.as_tons(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(MassUnit::Kilograms).fmt(formatter)
    }
}
