#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Velocity(pub(crate) f64);

impl Velocity
{
    pub const fn meters_per_second(value: f64) -> Self
    {
        Self(value)
    }

    pub const fn kilometers_per_hour(value: f64) -> Self
    {
        Self(value / 3.6)
    }

    pub const fn to_meters_per_second(self) -> f64
    {
        self.0
    }

    pub const fn to_kilometers_per_hour(self) -> f64
    {
        self.0 * 3.6
    }
}

pub const fn meters_per_second(value: f64) -> Velocity
{
    Velocity::meters_per_second(value)
}

pub const fn kilometers_per_hour(value: f64) -> Velocity
{
    Velocity::kilometers_per_hour(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VelocityUnit
{
    MetersPerSecond,
    KilometersPerHour,
}

impl VelocityUnit
{
    pub const fn symbol(self) -> &'static str
    {
        match self
        {
            Self::MetersPerSecond => "m/s",
            Self::KilometersPerHour => "km/h",
        }
    }
}
