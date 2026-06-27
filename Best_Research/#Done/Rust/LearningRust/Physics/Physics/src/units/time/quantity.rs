use crate::internal::{QuantityError, validate_finite};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Time(pub(crate) f64);

impl Time
{
    pub const fn seconds(value: f64) -> Self
    {
        Self(value)
    }

    pub const fn milliseconds(value: f64) -> Self
    {
        Self(value / 1_000.0)
    }

    pub const fn microseconds(value: f64) -> Self
    {
        Self(value / 1_000_000.0)
    }

    pub const fn nanoseconds(value: f64) -> Self
    {
        Self(value / 1_000_000_000.0)
    }

    pub const fn minutes(value: f64) -> Self
    {
        Self(value * 60.0)
    }

    pub const fn hours(value: f64) -> Self
    {
        Self(value * 3_600.0)
    }

    pub fn try_seconds(value: f64) -> Result<Self, QuantityError>
    {
        Ok(Self(validate_finite("Time", "s", value)?))
    }

    pub const fn to_seconds(self) -> f64
    {
        self.0
    }

    pub const fn to_milliseconds(self) -> f64
    {
        self.0 * 1_000.0
    }

    pub const fn to_microseconds(self) -> f64
    {
        self.0 * 1_000_000.0
    }

    pub const fn to_nanoseconds(self) -> f64
    {
        self.0 * 1_000_000_000.0
    }

    pub const fn to_minutes(self) -> f64
    {
        self.0 / 60.0
    }

    pub const fn to_hours(self) -> f64
    {
        self.0 / 3_600.0
    }
}

pub const fn seconds(value: f64) -> Time
{
    Time::seconds(value)
}

pub const fn milliseconds(value: f64) -> Time
{
    Time::milliseconds(value)
}

pub const fn microseconds(value: f64) -> Time
{
    Time::microseconds(value)
}

pub const fn nanoseconds(value: f64) -> Time
{
    Time::nanoseconds(value)
}

pub const fn minutes(value: f64) -> Time
{
    Time::minutes(value)
}

pub const fn hours(value: f64) -> Time
{
    Time::hours(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimeUnit
{
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Minutes,
    Hours,
}

impl TimeUnit
{
    pub const fn symbol(self) -> &'static str
    {
        match self
        {
            Self::Seconds => "s",
            Self::Milliseconds => "ms",
            Self::Microseconds => "us",
            Self::Nanoseconds => "ns",
            Self::Minutes => "min",
            Self::Hours => "h",
        }
    }
}
