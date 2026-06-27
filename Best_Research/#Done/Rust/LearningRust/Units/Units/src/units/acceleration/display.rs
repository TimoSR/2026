use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Acceleration, AccelerationUnit};

impl Acceleration {
    #[must_use]
    pub const fn display_as(self, unit: AccelerationUnit) -> AccelerationDisplay {
        AccelerationDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(
        self,
        unit: AccelerationUnit,
        precision: usize,
    ) -> AccelerationDisplay {
        AccelerationDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AccelerationDisplay {
    value: Acceleration,
    unit: AccelerationUnit,
    precision: Option<usize>,
}

impl fmt::Display for AccelerationDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            AccelerationUnit::MetersPerSecondSquared => self.value.as_meters_per_second_squared(),
            AccelerationUnit::StandardGravity => self.value.as_standard_gravity(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(AccelerationUnit::MetersPerSecondSquared)
            .fmt(formatter)
    }
}
