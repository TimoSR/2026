#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Acceleration(pub(crate) f64);

impl Acceleration
{
    pub const STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED: f64 = 9.80665;

    #[must_use]
    pub const fn meters_per_second_squared(value: f64) -> Self
    {
        Self(value)
    }

    #[must_use]
    pub const fn standard_gravity(value: f64) -> Self
    {
        Self(value * Self::STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED)
    }

    #[must_use]
    pub const fn to_meters_per_second_squared(self) -> f64
    {
        self.0
    }

    #[must_use]
    pub const fn to_standard_gravity(self) -> f64
    {
        self.0 / Self::STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED
    }
}

#[must_use]
pub const fn meters_per_second_squared(value: f64) -> Acceleration
{
    Acceleration::meters_per_second_squared(value)
}

#[must_use]
pub const fn standard_gravity(value: f64) -> Acceleration
{
    Acceleration::standard_gravity(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AccelerationUnit
{
    MetersPerSecondSquared,
    StandardGravity,
}

impl AccelerationUnit
{
    #[must_use]
    pub const fn symbol(self) -> &'static str
    {
        match self
        {
            Self::MetersPerSecondSquared => "m/s^2",
            Self::StandardGravity => "g0",
        }
    }
}
