use std::fmt;

use crate::internal::{
    QuantityError, check_nonzero, format_unit_value, implement_quantity_arithmetic,
};
use crate::{Acceleration, Mass};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Force(pub(crate) f64);

impl Force {
    #[must_use]
    pub const fn newtons(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn millinewtons(value: f64) -> Self {
        Self(value / 1_000.0)
    }

    #[must_use]
    pub const fn kilonewtons(value: f64) -> Self {
        Self(value * 1_000.0)
    }

    #[must_use]
    pub const fn as_newtons(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_millinewtons(self) -> f64 {
        self.0 * 1_000.0
    }

    #[must_use]
    pub const fn as_kilonewtons(self) -> f64 {
        self.0 / 1_000.0
    }

    #[must_use]
    pub const fn display_as(self, unit: ForceUnit) -> ForceDisplay {
        ForceDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(self, unit: ForceUnit, precision: usize) -> ForceDisplay {
        ForceDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub fn checked_div_mass(self, mass: Mass) -> Result<Acceleration, QuantityError> {
        check_nonzero(mass.as_kilograms(), "Force / Mass")?;
        Ok(self / mass)
    }
}

#[must_use]
pub const fn newtons(value: f64) -> Force {
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

implement_quantity_arithmetic!(Force);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ForceUnit {
    Newtons,
    Millinewtons,
    Kilonewtons,
}

impl ForceUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Newtons => "N",
            Self::Millinewtons => "mN",
            Self::Kilonewtons => "kN",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ForceDisplay {
    value: Force,
    unit: ForceUnit,
    precision: Option<usize>,
}

impl fmt::Display for ForceDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            ForceUnit::Newtons => self.value.as_newtons(),
            ForceUnit::Millinewtons => self.value.as_millinewtons(),
            ForceUnit::Kilonewtons => self.value.as_kilonewtons(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Force {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(ForceUnit::Newtons).fmt(formatter)
    }
}
