use std::fmt;

use crate::internal::format_unit_value;

use super::quantity::{Velocity, VelocityUnit};

impl Velocity
{
    pub(crate) const fn display_as(self, unit: VelocityUnit) -> VelocityDisplay
    {
        VelocityDisplay { value: self, unit, precision: None }
    }

    pub(crate) const fn display_as_precision(self, unit: VelocityUnit, precision: usize) -> VelocityDisplay
    {
        VelocityDisplay {
            value: self,
            unit,
            precision: Some(precision),
        }
    }

    pub const fn display_meters_per_second(self) -> VelocityDisplay
    {
        self.display_as(VelocityUnit::MetersPerSecond)
    }

    pub const fn display_kilometers_per_hour(self) -> VelocityDisplay
    {
        self.display_as(VelocityUnit::KilometersPerHour)
    }

    pub const fn display_meters_per_second_precision(self, precision: usize) -> VelocityDisplay
    {
        self.display_as_precision(VelocityUnit::MetersPerSecond, precision)
    }

    pub const fn display_kilometers_per_hour_precision(self, precision: usize) -> VelocityDisplay
    {
        self.display_as_precision(VelocityUnit::KilometersPerHour, precision)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VelocityDisplay
{
    value: Velocity,
    unit: VelocityUnit,
    precision: Option<usize>,
}

impl fmt::Display for VelocityDisplay
{
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result
    {
        let value = match self.unit
        {
            VelocityUnit::MetersPerSecond => self.value.to_meters_per_second(),
            VelocityUnit::KilometersPerHour => self.value.to_kilometers_per_hour(),
        };

        format_unit_value(formatter, value, self.unit.symbol(), self.precision)
    }
}

impl fmt::Display for Velocity
{
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter<'formatter>) -> fmt::Result
    {
        self.display_meters_per_second().fmt(formatter)
    }
}
