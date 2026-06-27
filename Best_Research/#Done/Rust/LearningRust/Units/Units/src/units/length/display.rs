use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Length, LengthUnit};

impl Length {
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
