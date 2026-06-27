use std::fmt;

use crate::internal::{
    QuantityError, format_unit_value, implement_quantity_arithmetic, validate_finite,
};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Time(pub(crate) f64);

impl Time {
    #[must_use]
    pub const fn seconds(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn milliseconds(value: f64) -> Self {
        Self(value / 1_000.0)
    }

    #[must_use]
    pub const fn microseconds(value: f64) -> Self {
        Self(value / 1_000_000.0)
    }

    #[must_use]
    pub const fn nanoseconds(value: f64) -> Self {
        Self(value / 1_000_000_000.0)
    }

    #[must_use]
    pub const fn minutes(value: f64) -> Self {
        Self(value * 60.0)
    }

    #[must_use]
    pub const fn hours(value: f64) -> Self {
        Self(value * 3_600.0)
    }

    pub fn try_seconds(value: f64) -> Result<Self, QuantityError> {
        Ok(Self(validate_finite("Time", "s", value)?))
    }

    #[must_use]
    pub const fn as_seconds(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_milliseconds(self) -> f64 {
        self.0 * 1_000.0
    }

    #[must_use]
    pub const fn as_microseconds(self) -> f64 {
        self.0 * 1_000_000.0
    }

    #[must_use]
    pub const fn as_nanoseconds(self) -> f64 {
        self.0 * 1_000_000_000.0
    }

    #[must_use]
    pub const fn as_minutes(self) -> f64 {
        self.0 / 60.0
    }

    #[must_use]
    pub const fn as_hours(self) -> f64 {
        self.0 / 3_600.0
    }

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

#[must_use]
pub const fn seconds(value: f64) -> Time {
    Time::seconds(value)
}

#[must_use]
pub const fn milliseconds(value: f64) -> Time {
    Time::milliseconds(value)
}

#[must_use]
pub const fn microseconds(value: f64) -> Time {
    Time::microseconds(value)
}

#[must_use]
pub const fn nanoseconds(value: f64) -> Time {
    Time::nanoseconds(value)
}

#[must_use]
pub const fn minutes(value: f64) -> Time {
    Time::minutes(value)
}

#[must_use]
pub const fn hours(value: f64) -> Time {
    Time::hours(value)
}

implement_quantity_arithmetic!(Time);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimeUnit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Minutes,
    Hours,
}

impl TimeUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Seconds => "s",
            Self::Milliseconds => "ms",
            Self::Microseconds => "us",
            Self::Nanoseconds => "ns",
            Self::Minutes => "min",
            Self::Hours => "h",
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
