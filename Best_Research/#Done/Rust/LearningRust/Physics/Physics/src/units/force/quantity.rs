#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Force(pub(crate) f64);

impl Force
{
     
    pub const fn newtons(value: f64) -> Self
    {
        Self(value)
    }

     
    pub const fn millinewtons(value: f64) -> Self
    {
        Self(value / 1_000.0)
    }

     
    pub const fn kilonewtons(value: f64) -> Self
    {
        Self(value * 1_000.0)
    }

     
    pub const fn to_newtons(self) -> f64
    {
        self.0
    }

     
    pub const fn to_millinewtons(self) -> f64
    {
        self.0 * 1_000.0
    }

     
    pub const fn to_kilonewtons(self) -> f64
    {
        self.0 / 1_000.0
    }
}

 
pub const fn newtons(value: f64) -> Force
{
    Force::newtons(value)
}

 
pub const fn millinewtons(value: f64) -> Force
{
    Force::millinewtons(value)
}

 
pub const fn kilonewtons(value: f64) -> Force
{
    Force::kilonewtons(value)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ForceUnit
{
    Newtons,
    Millinewtons,
    Kilonewtons,
}

impl ForceUnit
{
     
    pub const fn symbol(self) -> &'static str
    {
        match self
        {
            Self::Newtons => "N",
            Self::Millinewtons => "mN",
            Self::Kilonewtons => "kN",
        }
    }
}
