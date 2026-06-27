#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Force(pub(crate) f64);

impl Force
{
    #[must_use]
    pub const fn newtons(value: f64) -> Self
    {
        Self(value)
    }

    #[must_use]
    pub const fn millinewtons(value: f64) -> Self
    {
        Self(value / 1_000.0)
    }

    #[must_use]
    pub const fn kilonewtons(value: f64) -> Self
    {
        Self(value * 1_000.0)
    }

    #[must_use]
    pub const fn to_newtons(self) -> f64
    {
        self.0
    }

    #[must_use]
    pub const fn to_millinewtons(self) -> f64
    {
        self.0 * 1_000.0
    }

    #[must_use]
    pub const fn to_kilonewtons(self) -> f64
    {
        self.0 / 1_000.0
    }
}

#[must_use]
pub const fn newtons(value: f64) -> Force
{
    Force::newtons(value)
}

#[must_use]
pub const fn millinewtons(value: f64) -> Force
{
    Force::millinewtons(value)
}

#[must_use]
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
    #[must_use]
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
