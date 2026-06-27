use crate::internal::{QuantityError, validate_finite};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Length(pub(crate) f64);

impl Length
{
     
    pub const fn meters(value: f64) -> Self
    {
        Self(value)
    }

     
    pub const fn kilometers(value: f64) -> Self
    {
        Self(value * 1_000.0)
    }

     
    pub const fn centimeters(value: f64) -> Self
    {
        Self(value / 100.0)
    }

     
    pub const fn millimeters(value: f64) -> Self
    {
        Self(value / 1_000.0)
    }

     
    pub const fn micrometers(value: f64) -> Self
    {
        Self(value / 1_000_000.0)
    }

     
    pub const fn nanometers(value: f64) -> Self
    {
        Self(value / 1_000_000_000.0)
    }

    pub fn try_meters(value: f64) -> Result<Self, QuantityError>
    {
        Ok(Self(validate_finite("Length", "m", value)?))
    }

    pub fn try_centimeters(value: f64) -> Result<Self, QuantityError>
    {
        Ok(Self(validate_finite("Length", "cm", value)? / 100.0))
    }

     
    pub const fn to_meters(self) -> f64
    {
        self.0
    }

     
    pub const fn to_kilometers(self) -> f64
    {
        self.0 / 1_000.0
    }

     
    pub const fn to_centimeters(self) -> f64
    {
        self.0 * 100.0
    }

     
    pub const fn to_millimeters(self) -> f64
    {
        self.0 * 1_000.0
    }

     
    pub const fn to_micrometers(self) -> f64
    {
        self.0 * 1_000_000.0
    }

     
    pub const fn to_nanometers(self) -> f64
    {
        self.0 * 1_000_000_000.0
    }
}

 
pub const fn meters(value: f64) -> Length
{
    Length::meters(value)
}

 
pub const fn kilometers(value: f64) -> Length
{
    Length::kilometers(value)
}

 
pub const fn centimeters(value: f64) -> Length
{
    Length::centimeters(value)
}

 
pub const fn millimeters(value: f64) -> Length
{
    Length::millimeters(value)
}

 
pub const fn micrometers(value: f64) -> Length
{
    Length::micrometers(value)
}

 
pub const fn nanometers(value: f64) -> Length
{
    Length::nanometers(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LengthUnit
{
    Meters,
    Kilometers,
    Centimeters,
    Millimeters,
    Micrometers,
    Nanometers,
}

impl LengthUnit
{
     
    pub const fn symbol(self) -> &'static str
    {
        match self
        {
            Self::Meters => "m",
            Self::Kilometers => "km",
            Self::Centimeters => "cm",
            Self::Millimeters => "mm",
            Self::Micrometers => "um",
            Self::Nanometers => "nm",
        }
    }
}
