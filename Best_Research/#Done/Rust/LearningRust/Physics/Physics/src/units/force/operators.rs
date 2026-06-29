use std::ops::Div;

use crate::QuantityError;
use crate::acceleration::Acceleration;
use crate::check_nonzero;
use crate::force::Force;
use crate::implement_quantity_arithmetic;
use crate::mass::Mass;

implement_quantity_arithmetic!(Force);

impl Force {
    pub fn checked_div_mass(self, mass: Mass) -> Result<Acceleration, QuantityError> {
        check_nonzero(mass.to_kilograms(), "Force / Mass")?;
        Ok(self / mass)
    }
}

impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, mass: Mass) -> Self::Output { Acceleration::meters_per_second_squared(self.to_newtons() / mass.to_kilograms()) }
}

impl Div<Acceleration> for Force {
    type Output = Mass;

    fn div(self, acceleration: Acceleration) -> Self::Output { Mass::kilograms(self.to_newtons() / acceleration.to_meters_per_second_squared()) }
}
