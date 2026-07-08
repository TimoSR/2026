use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::Length;
use super::quantity::LengthUnit;

impl Length {
    pub(crate) const fn display_as(self, unit: LengthUnit) -> LengthDisplay {
        LengthDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    pub(crate) const fn display_as_precision(self, unit: LengthUnit, precision: usize) -> LengthDisplay {
        LengthDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub const fn display_meters(self) -> LengthDisplay { self.display_as(LengthUnit::Meters) }

    pub const fn display_kilometers(self) -> LengthDisplay { self.display_as(LengthUnit::Kilometers) }

    pub const fn display_centimeters(self) -> LengthDisplay { self.display_as(LengthUnit::Centimeters) }

    pub const fn display_millimeters(self) -> LengthDisplay { self.display_as(LengthUnit::Millimeters) }

    pub const fn display_micrometers(self) -> LengthDisplay { self.display_as(LengthUnit::Micrometers) }

    pub const fn display_nanometers(self) -> LengthDisplay { self.display_as(LengthUnit::Nanometers) }

    pub const fn display_meters_precision(self, precision: usize) -> LengthDisplay { self.display_as_precision(LengthUnit::Meters, precision) }

    pub const fn display_kilometers_precision(self, precision: usize) -> LengthDisplay {
        self.display_as_precision(LengthUnit::Kilometers, precision)
    }

    pub const fn display_centimeters_precision(self, precision: usize) -> LengthDisplay {
        self.display_as_precision(LengthUnit::Centimeters, precision)
    }

    pub const fn display_millimeters_precision(self, precision: usize) -> LengthDisplay {
        self.display_as_precision(LengthUnit::Millimeters, precision)
    }

    pub const fn display_micrometers_precision(self, precision: usize) -> LengthDisplay {
        self.display_as_precision(LengthUnit::Micrometers, precision)
    }

    pub const fn display_nanometers_precision(self, precision: usize) -> LengthDisplay {
        self.display_as_precision(LengthUnit::Nanometers, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LengthDisplay {
    value: Length,
    unit: LengthUnit,
    precision: Option<usize>,
}

impl fmt::Display for LengthDisplay {
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result {
        let value = match self.unit {
            LengthUnit::Meters => self.value.to_meters(),
            LengthUnit::Kilometers => self.value.to_kilometers(),
            LengthUnit::Centimeters => self.value.to_centimeters(),
            LengthUnit::Millimeters => self.value.to_millimeters(),
            LengthUnit::Micrometers => self.value.to_micrometers(),
            LengthUnit::Nanometers => self.value.to_nanometers(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Length {
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result { self.display_as(LengthUnit::Meters).fmt(formatter) }
}
