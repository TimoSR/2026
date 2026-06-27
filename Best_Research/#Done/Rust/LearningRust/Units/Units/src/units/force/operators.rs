use std::ops::Div;

use crate::internal::{QuantityError, check_nonzero, implement_quantity_arithmetic};
use crate::{Acceleration, Mass};

use super::quantity::Force;

implement_quantity_arithmetic!(Force);

impl Force {
    pub fn checked_div_mass(self, mass: Mass) -> Result<Acceleration, QuantityError> {
        check_nonzero(mass.as_kilograms(), "Force / Mass")?;
        Ok(self / mass)
    }
}

impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, mass: Mass) -> Self::Output {
        Acceleration::meters_per_second_squared(self.as_newtons() / mass.as_kilograms())
    }
}

impl Div<Acceleration> for Force {
    type Output = Mass;

    fn div(self, acceleration: Acceleration) -> Self::Output {
        Mass::kilograms(self.as_newtons() / acceleration.as_meters_per_second_squared())
    }
}
