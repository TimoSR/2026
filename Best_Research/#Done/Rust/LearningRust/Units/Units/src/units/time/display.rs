use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Time, TimeUnit};

impl Time {
    #[must_use]
    pub const fn display_as(self, unit: TimeUnit) -> TimeDisplay {
        TimeDisplay {
            value: self,
            unit,
            precision: None,
        }
    }

    #[must_use]
    pub const fn display_as_precision(self, unit: TimeUnit, precision: usize) -> TimeDisplay {
        TimeDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TimeDisplay {
    value: Time,
    unit: TimeUnit,
    precision: Option<usize>,
}

impl fmt::Display for TimeDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit {
            TimeUnit::Seconds => self.value.as_seconds(),
            TimeUnit::Milliseconds => self.value.as_milliseconds(),
            TimeUnit::Microseconds => self.value.as_microseconds(),
            TimeUnit::Nanoseconds => self.value.as_nanoseconds(),
            TimeUnit::Minutes => self.value.as_minutes(),
            TimeUnit::Hours => self.value.as_hours(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_as(TimeUnit::Seconds).fmt(formatter)
    }
}
