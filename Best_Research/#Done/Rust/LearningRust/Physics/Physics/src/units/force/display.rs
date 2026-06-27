use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Force, ForceUnit};

impl Force
{
     
    pub(crate) const fn display_as(self, unit: ForceUnit) -> ForceDisplay
    {
        ForceDisplay { value: self, unit, precision: None }
    }

     
    pub(crate) const fn display_as_precision(self, unit: ForceUnit, precision: usize) -> ForceDisplay
    {
        ForceDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

     
    pub const fn display_newtons(self) -> ForceDisplay
    {
        self.display_as(ForceUnit::Newtons)
    }

     
    pub const fn display_millinewtons(self) -> ForceDisplay
    {
        self.display_as(ForceUnit::Millinewtons)
    }

     
    pub const fn display_kilonewtons(self) -> ForceDisplay
    {
        self.display_as(ForceUnit::Kilonewtons)
    }

     
    pub const fn display_newtons_precision(self, precision: usize) -> ForceDisplay
    {
        self.display_as_precision(ForceUnit::Newtons, precision)
    }

     
    pub const fn display_millinewtons_precision(self, precision: usize) -> ForceDisplay
    {
        self.display_as_precision(ForceUnit::Millinewtons, precision)
    }

     
    pub const fn display_kilonewtons_precision(self, precision: usize) -> ForceDisplay
    {
        self.display_as_precision(ForceUnit::Kilonewtons, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ForceDisplay
{
    value: Force,
    unit: ForceUnit,
    precision: Option<usize>,
}

impl fmt::Display for ForceDisplay
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let value = match self.unit
        {
            ForceUnit::Newtons => self.value.to_newtons(),
            ForceUnit::Millinewtons => self.value.to_millinewtons(),
            ForceUnit::Kilonewtons => self.value.to_kilonewtons(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Force
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        self.display_newtons().fmt(formatter)
    }
}
