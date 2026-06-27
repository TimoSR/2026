use crate::{Acceleration, Mass};

#[test]
fn force_calculates_mass_times_acceleration()
{
    let force = crate::force(
        Mass::kilograms(3.0),
        Acceleration::meters_per_second_squared(4.0),
    );

    super::assert_close(force.to_newtons(), 12.0);
}
