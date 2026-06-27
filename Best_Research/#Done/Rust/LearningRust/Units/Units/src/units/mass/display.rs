use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Mass, MassUnit};

impl Mass {
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
