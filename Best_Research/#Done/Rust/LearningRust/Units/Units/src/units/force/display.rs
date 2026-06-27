use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Force, ForceUnit};

impl Force {
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
