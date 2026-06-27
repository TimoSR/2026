use crate::internal::{QuantityError, validate_finite};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Mass(pub(crate) f64);

impl Mass
{
     
    pub const fn kilograms(value: f64) -> Self
    {
        Self(value)
    }

     
    pub const fn kilogram(value: f64) -> Self
    {
        Self::kilograms(value)
    }

     
    pub const fn grams(value: f64) -> Self
    {
        Self(value / 1_000.0)
    }

     
    pub const fn milligrams(value: f64) -> Self
    {
        Self(value / 1_000_000.0)
    }

     
    pub const fn micrograms(value: f64) -> Self
    {
        Self(value / 1_000_000_000.0)
    }

     
    pub const fn tons(value: f64) -> Self
    {
        Self(value * 1_000.0)
    }

    pub fn try_kilograms(value: f64) -> Result<Self, QuantityError>
    {
        Ok(Self(validate_finite("Mass", "kg", value)?))
    }

     
    pub const fn to_kilograms(self) -> f64
    {
        self.0
    }

     
    pub const fn to_grams(self) -> f64
    {
        self.0 * 1_000.0
    }

     
    pub const fn to_milligrams(self) -> f64
    {
        self.0 * 1_000_000.0
    }

     
    pub const fn to_micrograms(self) -> f64
    {
        self.0 * 1_000_000_000.0
    }

     
    pub const fn to_tons(self) -> f64
    {
        self.0 / 1_000.0
    }
}

 
pub const fn kilograms(value: f64) -> Mass
{
    Mass::kilograms(value)
}

 
pub const fn kilogram(value: f64) -> Mass
{
    Mass::kilogram(value)
}

 
pub const fn grams(value: f64) -> Mass
{
    Mass::grams(value)
}

 
pub const fn milligrams(value: f64) -> Mass
{
    Mass::milligrams(value)
}

 
pub const fn micrograms(value: f64) -> Mass
{
    Mass::micrograms(value)
}

 
pub const fn tons(value: f64) -> Mass
{
    Mass::tons(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MassUnit
{
    Kilograms,
    Grams,
    Milligrams,
    Micrograms,
    Tons,
}

impl MassUnit
{
     
    pub const fn symbol(self) -> &'static str
    {
        match self
        {
            Self::Kilograms => "kg",
            Self::Grams => "g",
            Self::Milligrams => "mg",
            Self::Micrograms => "ug",
            Self::Tons => "t",
        }
    }
}
