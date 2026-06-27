use crate::{Time, Velocity};

#[test]
fn acceleration_calculates_velocity_divided_by_time()
{
    let acceleration = crate::acceleration(
        Velocity::meters_per_second(12.0),
        Time::seconds(4.0),
    );

    super::assert_close(acceleration.to_meters_per_second_squared(), 3.0);
}
