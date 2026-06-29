use crate::Acceleration;
use crate::Length;
use crate::Time;

use super::super::*;

#[test]
fn velocity_multiplied_by_time_returns_length() {
    let length = meters_per_second(3.0) * Time::seconds(4.0);

    super::assert_close(length.to_meters(), 12.0);
}

#[test]
fn calculate_returns_distance_divided_by_time() {
    let velocity = calculate(Length::meters(100.0), Time::seconds(10.0));

    super::assert_close(velocity.to_meters_per_second(), 10.0);
}

#[test]
fn velocity_divided_by_time_returns_acceleration() {
    let acceleration = meters_per_second(12.0) / Time::seconds(4.0);

    super::assert_close(acceleration.to_meters_per_second_squared(), 3.0);
}

#[test]
fn velocity_divided_by_acceleration_returns_time() {
    let time = meters_per_second(12.0) / Acceleration::meters_per_second_squared(3.0);

    super::assert_close(time.to_seconds(), 4.0);
}

#[test]
fn checked_div_time_rejects_zero_time() {
    let result = meters_per_second(12.0).checked_div_time(Time::seconds(0.0));

    assert!(result.is_err());
}

#[test]
fn checked_calculate_rejects_zero_time() {
    let result = checked_calculate(Length::meters(100.0), Time::seconds(0.0));

    assert!(result.is_err());
}
