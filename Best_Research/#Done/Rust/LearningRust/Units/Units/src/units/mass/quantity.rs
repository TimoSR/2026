use crate::internal::{QuantityError, validate_finite};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Mass(pub(crate) f64);

impl Mass {
    #[must_use]
    pub const fn kilograms(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn kilogram(value: f64) -> Self {
        Self::kilograms(value)
    }

    #[must_use]
    pub const fn grams(value: f64) -> Self {
        Self(value / 1_000.0)
    }

    #[must_use]
    pub const fn milligrams(value: f64) -> Self {
        Self(value / 1_000_000.0)
    }

    #[must_use]
    pub const fn micrograms(value: f64) -> Self {
        Self(value / 1_000_000_000.0)
    }

    #[must_use]
    pub const fn tons(value: f64) -> Self {
        Self(value * 1_000.0)
    }

    pub fn try_kilograms(value: f64) -> Result<Self, QuantityError> {
        Ok(Self(validate_finite("Mass", "kg", value)?))
    }

    #[must_use]
    pub const fn to_kilograms(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn to_grams(self) -> f64 {
        self.0 * 1_000.0
    }

    #[must_use]
    pub const fn to_milligrams(self) -> f64 {
        self.0 * 1_000_000.0
    }

    #[must_use]
    pub const fn to_micrograms(self) -> f64 {
        self.0 * 1_000_000_000.0
    }

    #[must_use]
    pub const fn to_tons(self) -> f64 {
        self.0 / 1_000.0
    }
}

#[must_use]
pub const fn kilograms(value: f64) -> Mass {
    Mass::kilograms(value)
}

#[must_use]
pub const fn kilogram(value: f64) -> Mass {
    Mass::kilogram(value)
}

#[must_use]
pub const fn grams(value: f64) -> Mass {
    Mass::grams(value)
}

#[must_use]
pub const fn milligrams(value: f64) -> Mass {
    Mass::milligrams(value)
}

#[must_use]
pub const fn micrograms(value: f64) -> Mass {
    Mass::micrograms(value)
}

#[must_use]
pub const fn tons(value: f64) -> Mass {
    Mass::tons(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MassUnit {
    Kilograms,
    Grams,
    Milligrams,
    Micrograms,
    Tons,
}

impl MassUnit {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Kilograms => "kg",
            Self::Grams => "g",
            Self::Milligrams => "mg",
            Self::Micrograms => "ug",
            Self::Tons => "t",
        }
    }
}
