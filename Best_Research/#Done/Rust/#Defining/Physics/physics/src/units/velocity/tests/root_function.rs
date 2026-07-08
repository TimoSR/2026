use crate::Length;
use crate::Time;

#[test]
fn velocity_calculates_distance_divided_by_time() {
    let velocity = crate::velocity(Length::meters(100.0), Time::seconds(10.0));

    super::assert_close(velocity.to_meters_per_second(), 10.0);
}
