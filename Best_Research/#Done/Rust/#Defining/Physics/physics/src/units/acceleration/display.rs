use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::Acceleration;
use super::quantity::AccelerationUnit;

impl Acceleration {
    pub(crate) const fn display_as(self, unit: AccelerationUnit) -> AccelerationDisplay {
        AccelerationDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    pub(crate) const fn display_as_precision(self, unit: AccelerationUnit, precision: usize) -> AccelerationDisplay {
        AccelerationDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub const fn display_meters_per_second_squared(self) -> AccelerationDisplay { self.display_as(AccelerationUnit::MetersPerSecondSquared) }

    pub const fn display_standard_gravity(self) -> AccelerationDisplay { self.display_as(AccelerationUnit::StandardGravity) }

    pub const fn display_meters_per_second_squared_precision(self, precision: usize) -> AccelerationDisplay {
        self.display_as_precision(AccelerationUnit::MetersPerSecondSquared, precision)
    }

    pub const fn display_standard_gravity_precision(self, precision: usize) -> AccelerationDisplay {
        self.display_as_precision(AccelerationUnit::StandardGravity, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AccelerationDisplay {
    value: Acceleration,
    unit: AccelerationUnit,
    precision: Option<usize>,
}

impl fmt::Display for AccelerationDisplay {
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result {
        let value = match self.unit {
            AccelerationUnit::MetersPerSecondSquared => self.value.to_meters_per_second_squared(),
            AccelerationUnit::StandardGravity => self.value.to_standard_gravity(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Acceleration {
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result { self.display_meters_per_second_squared().fmt(formatter) }
}
