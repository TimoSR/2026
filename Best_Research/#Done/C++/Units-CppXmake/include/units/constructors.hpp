#pragma once

#include "units/acceleration.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units::length {

[[nodiscard]] Length meters(double value);
[[nodiscard]] Length kilometers(double value);
[[nodiscard]] Length centimeters(double value);
[[nodiscard]] Length millimeters(double value);
[[nodiscard]] Length micrometers(double value);
[[nodiscard]] Length nanometers(double value);

} // namespace units::length

namespace units::time {

[[nodiscard]] Time seconds(double value);
[[nodiscard]] Time milliseconds(double value);
[[nodiscard]] Time microseconds(double value);
[[nodiscard]] Time nanoseconds(double value);
[[nodiscard]] Time minutes(double value);
[[nodiscard]] Time hours(double value);

} // namespace units::time

namespace units::mass {

[[nodiscard]] Mass kilograms(double value);
[[nodiscard]] Mass kilogram(double value);
[[nodiscard]] Mass grams(double value);
[[nodiscard]] Mass milligrams(double value);
[[nodiscard]] Mass micrograms(double value);
[[nodiscard]] Mass tons(double value);

} // namespace units::mass

namespace units::velocity {

[[nodiscard]] Velocity metersPerSecond(double value);
[[nodiscard]] Velocity meters_pr_second(double value);
[[nodiscard]] Velocity kilometersPerHour(double value);

} // namespace units::velocity

namespace units::acceleration {

[[nodiscard]] Acceleration metersPerSecondSquared(double value);
[[nodiscard]] Acceleration meters_pr_second_pr_second(double value);
[[nodiscard]] Acceleration standardGravity(double value);

} // namespace units::acceleration

namespace units::force {

[[nodiscard]] Force newtons(double value);
[[nodiscard]] Force force(double value);
[[nodiscard]] Force millinewtons(double value);
[[nodiscard]] Force kilonewtons(double value);

} // namespace units::force
