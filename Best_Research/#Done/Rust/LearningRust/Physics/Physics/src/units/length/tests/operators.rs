use crate::Time;
use crate::Velocity;

use super::super::*;

#[test]
fn length_divided_by_time_returns_velocity() {
    let velocity = meters(100.0) / Time::seconds(10.0);

    super::assert_close(velocity.to_meters_per_second(), 10.0);
}

#[test]
fn length_divided_by_velocity_returns_time() {
    let time = meters(100.0) / Velocity::meters_per_second(10.0);

    super::assert_close(time.to_seconds(), 10.0);
}

#[test]
fn checked_div_time_rejects_zero_time() {
    let result = meters(100.0).checked_div_time(Time::seconds(0.0));

    assert!(result.is_err());
}
