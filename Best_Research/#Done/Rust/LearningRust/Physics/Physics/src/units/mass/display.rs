use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Mass, MassUnit};

impl Mass
{
    #[must_use]
    pub(crate) const fn display_as(self, unit: MassUnit) -> MassDisplay
    {
        MassDisplay { value: self, unit, precision: None }
    }

    #[must_use]
    pub(crate) const fn display_as_precision(self, unit: MassUnit, precision: usize) -> MassDisplay
    {
        MassDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    #[must_use]
    pub const fn display_kilograms(self) -> MassDisplay
    {
        self.display_as(MassUnit::Kilograms)
    }

    #[must_use]
    pub const fn display_grams(self) -> MassDisplay
    {
        self.display_as(MassUnit::Grams)
    }

    #[must_use]
    pub const fn display_milligrams(self) -> MassDisplay
    {
        self.display_as(MassUnit::Milligrams)
    }

    #[must_use]
    pub const fn display_micrograms(self) -> MassDisplay
    {
        self.display_as(MassUnit::Micrograms)
    }

    #[must_use]
    pub const fn display_tons(self) -> MassDisplay
    {
        self.display_as(MassUnit::Tons)
    }

    #[must_use]
    pub const fn display_kilograms_precision(self, precision: usize) -> MassDisplay
    {
        self.display_as_precision(MassUnit::Kilograms, precision)
    }

    #[must_use]
    pub const fn display_grams_precision(self, precision: usize) -> MassDisplay
    {
        self.display_as_precision(MassUnit::Grams, precision)
    }

    #[must_use]
    pub const fn display_milligrams_precision(self, precision: usize) -> MassDisplay
    {
        self.display_as_precision(MassUnit::Milligrams, precision)
    }

    #[must_use]
    pub const fn display_micrograms_precision(self, precision: usize) -> MassDisplay
    {
        self.display_as_precision(MassUnit::Micrograms, precision)
    }

    #[must_use]
    pub const fn display_tons_precision(self, precision: usize) -> MassDisplay
    {
        self.display_as_precision(MassUnit::Tons, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MassDisplay
{
    value: Mass,
    unit: MassUnit,
    precision: Option<usize>,
}

impl fmt::Display for MassDisplay
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let value = match self.unit
        {
            MassUnit::Kilograms => self.value.to_kilograms(),
            MassUnit::Grams => self.value.to_grams(),
            MassUnit::Milligrams => self.value.to_milligrams(),
            MassUnit::Micrograms => self.value.to_micrograms(),
            MassUnit::Tons => self.value.to_tons(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Mass
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        self.display_kilograms().fmt(formatter)
    }
}
