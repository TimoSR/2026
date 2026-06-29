use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::Mass;
use super::quantity::MassUnit;

impl Mass {
    pub(crate) const fn display_as(self, unit: MassUnit) -> MassDisplay {
        MassDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    pub(crate) const fn display_as_precision(self, unit: MassUnit, precision: usize) -> MassDisplay {
        MassDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub const fn display_kilograms(self) -> MassDisplay { self.display_as(MassUnit::Kilograms) }

    pub const fn display_grams(self) -> MassDisplay { self.display_as(MassUnit::Grams) }

    pub const fn display_milligrams(self) -> MassDisplay { self.display_as(MassUnit::Milligrams) }

    pub const fn display_micrograms(self) -> MassDisplay { self.display_as(MassUnit::Micrograms) }

    pub const fn display_tons(self) -> MassDisplay { self.display_as(MassUnit::Tons) }

    pub const fn display_kilograms_precision(self, precision: usize) -> MassDisplay { self.display_as_precision(MassUnit::Kilograms, precision) }

    pub const fn display_grams_precision(self, precision: usize) -> MassDisplay { self.display_as_precision(MassUnit::Grams, precision) }

    pub const fn display_milligrams_precision(self, precision: usize) -> MassDisplay { self.display_as_precision(MassUnit::Milligrams, precision) }

    pub const fn display_micrograms_precision(self, precision: usize) -> MassDisplay { self.display_as_precision(MassUnit::Micrograms, precision) }

    pub const fn display_tons_precision(self, precision: usize) -> MassDisplay { self.display_as_precision(MassUnit::Tons, precision) }
}

#[derive(Copy, Clone, Debug)]
pub struct MassDisplay {
    value: Mass,
    unit: MassUnit,
    precision: Option<usize>,
}

impl fmt::Display for MassDisplay {
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result {
        let value = match self.unit {
            MassUnit::Kilograms => self.value.to_kilograms(),
            MassUnit::Grams => self.value.to_grams(),
            MassUnit::Milligrams => self.value.to_milligrams(),
            MassUnit::Micrograms => self.value.to_micrograms(),
            MassUnit::Tons => self.value.to_tons(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Mass {
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result { self.display_kilograms().fmt(formatter) }
}
