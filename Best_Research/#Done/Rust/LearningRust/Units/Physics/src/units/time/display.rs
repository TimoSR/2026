use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Time, TimeUnit};

impl Time
{
    #[must_use]
    pub(crate) const fn display_as(self, unit: TimeUnit) -> TimeDisplay
    {
        TimeDisplay { value: self, unit, precision: None }
    }

    #[must_use]
    pub(crate) const fn display_as_precision(self, unit: TimeUnit, precision: usize) -> TimeDisplay
    {
        TimeDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    #[must_use]
    pub const fn display_seconds(self) -> TimeDisplay
    {
        self.display_as(TimeUnit::Seconds)
    }

    #[must_use]
    pub const fn display_milliseconds(self) -> TimeDisplay
    {
        self.display_as(TimeUnit::Milliseconds)
    }

    #[must_use]
    pub const fn display_microseconds(self) -> TimeDisplay
    {
        self.display_as(TimeUnit::Microseconds)
    }

    #[must_use]
    pub const fn display_nanoseconds(self) -> TimeDisplay
    {
        self.display_as(TimeUnit::Nanoseconds)
    }

    #[must_use]
    pub const fn display_minutes(self) -> TimeDisplay
    {
        self.display_as(TimeUnit::Minutes)
    }

    #[must_use]
    pub const fn display_hours(self) -> TimeDisplay
    {
        self.display_as(TimeUnit::Hours)
    }

    #[must_use]
    pub const fn display_seconds_precision(self, precision: usize) -> TimeDisplay
    {
        self.display_as_precision(TimeUnit::Seconds, precision)
    }

    #[must_use]
    pub const fn display_milliseconds_precision(self, precision: usize) -> TimeDisplay
    {
        self.display_as_precision(TimeUnit::Milliseconds, precision)
    }

    #[must_use]
    pub const fn display_microseconds_precision(self, precision: usize) -> TimeDisplay
    {
        self.display_as_precision(TimeUnit::Microseconds, precision)
    }

    #[must_use]
    pub const fn display_nanoseconds_precision(self, precision: usize) -> TimeDisplay
    {
        self.display_as_precision(TimeUnit::Nanoseconds, precision)
    }

    #[must_use]
    pub const fn display_minutes_precision(self, precision: usize) -> TimeDisplay
    {
        self.display_as_precision(TimeUnit::Minutes, precision)
    }

    #[must_use]
    pub const fn display_hours_precision(self, precision: usize) -> TimeDisplay
    {
        self.display_as_precision(TimeUnit::Hours, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TimeDisplay
{
    value: Time,
    unit: TimeUnit,
    precision: Option<usize>,
}

impl fmt::Display for TimeDisplay
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let value = match self.unit
        {
            TimeUnit::Seconds => self.value.to_seconds(),
            TimeUnit::Milliseconds => self.value.to_milliseconds(),
            TimeUnit::Microseconds => self.value.to_microseconds(),
            TimeUnit::Nanoseconds => self.value.to_nanoseconds(),
            TimeUnit::Minutes => self.value.to_minutes(),
            TimeUnit::Hours => self.value.to_hours(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Time
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        self.display_seconds().fmt(formatter)
    }
}
