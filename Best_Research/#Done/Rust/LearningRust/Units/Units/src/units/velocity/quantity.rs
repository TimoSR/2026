#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Velocity(pub(crate) f64);

impl Velocity {
    #[must_use]
    pub const fn meters_per_second(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn kilometers_per_hour(value: f64) -> Self {
        Self(value / 3.6)
    }

    #[must_use]
    pub const fn as_meters_per_second(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn as_kilometers_per_hour(self) -> f64 {
        self.0 * 3.6
    }
}

#[must_use]
pub const fn meters_per_second(value: f64) -> Velocity {
    Velocity::meters_per_second(value)
}

#[must_use]
pub const fn kilometers_per_hour(value: f64) -> Velocity {
    Velocity::kilometers_per_hour(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VelocityUnit {
    MetersPerSecond,
    KilometersPerHour,
}

impl VelocityUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::MetersPerSecond => "m/s",
            Self::KilometersPerHour => "km/h",
        }
    }
}
