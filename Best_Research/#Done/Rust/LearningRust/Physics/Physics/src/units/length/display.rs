use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Length, LengthUnit};

impl Length
{
    #[must_use]
    pub(crate) const fn display_as(self, unit: LengthUnit) -> LengthDisplay
    {
        LengthDisplay { value: self, unit, precision: None }
    }

    #[must_use]
    pub(crate) const fn display_as_precision(self, unit: LengthUnit, precision: usize) -> LengthDisplay
    {
        LengthDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    #[must_use]
    pub const fn display_meters(self) -> LengthDisplay
    {
        self.display_as(LengthUnit::Meters)
    }

    #[must_use]
    pub const fn display_kilometers(self) -> LengthDisplay
    {
        self.display_as(LengthUnit::Kilometers)
    }

    #[must_use]
    pub const fn display_centimeters(self) -> LengthDisplay
    {
        self.display_as(LengthUnit::Centimeters)
    }

    #[must_use]
    pub const fn display_millimeters(self) -> LengthDisplay
    {
        self.display_as(LengthUnit::Millimeters)
    }

    #[must_use]
    pub const fn display_micrometers(self) -> LengthDisplay
    {
        self.display_as(LengthUnit::Micrometers)
    }

    #[must_use]
    pub const fn display_nanometers(self) -> LengthDisplay
    {
        self.display_as(LengthUnit::Nanometers)
    }

    #[must_use]
    pub const fn display_meters_precision(self, precision: usize) -> LengthDisplay
    {
        self.display_as_precision(LengthUnit::Meters, precision)
    }

    #[must_use]
    pub const fn display_kilometers_precision(self, precision: usize) -> LengthDisplay
    {
        self.display_as_precision(LengthUnit::Kilometers, precision)
    }

    #[must_use]
    pub const fn display_centimeters_precision(self, precision: usize) -> LengthDisplay
    {
        self.display_as_precision(LengthUnit::Centimeters, precision)
    }

    #[must_use]
    pub const fn display_millimeters_precision(self, precision: usize) -> LengthDisplay
    {
        self.display_as_precision(LengthUnit::Millimeters, precision)
    }

    #[must_use]
    pub const fn display_micrometers_precision(self, precision: usize) -> LengthDisplay
    {
        self.display_as_precision(LengthUnit::Micrometers, precision)
    }

    #[must_use]
    pub const fn display_nanometers_precision(self, precision: usize) -> LengthDisplay
    {
        self.display_as_precision(LengthUnit::Nanometers, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LengthDisplay
{
    value: Length,
    unit: LengthUnit,
    precision: Option<usize>,
}

impl fmt::Display for LengthDisplay
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let value = match self.unit
        {
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

impl fmt::Display for Length
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        self.display_as(LengthUnit::Meters).fmt(formatter)
    }
}
