#pragma once

#include "units/acceleration.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units::length
{

    Length meters(double value);
    Length kilometers(double value);
    Length centimeters(double value);
    Length millimeters(double value);
    Length micrometers(double value);
    Length nanometers(double value);

} // namespace units::length

namespace units::time
{

    Time seconds(double value);
    Time milliseconds(double value);
    Time microseconds(double value);
    Time nanoseconds(double value);
    Time minutes(double value);
    Time hours(double value);

} // namespace units::time

namespace units::mass
{

    Mass kilograms(double value);
    Mass kilogram(double value);
    Mass grams(double value);
    Mass milligrams(double value);
    Mass micrograms(double value);
    Mass tons(double value);

} // namespace units::mass

namespace units::velocity
{

    Velocity metersPerSecond(double value);
    Velocity meters_pr_second(double value);
    Velocity kilometersPerHour(double value);

} // namespace units::velocity

namespace units::acceleration
{

    Acceleration metersPerSecondSquared(double value);
    Acceleration meters_pr_second_pr_second(double value);
    Acceleration standardGravity(double value);

} // namespace units::acceleration

namespace units::force
{

    Force newtons(double value);
    Force force(double value);
    Force millinewtons(double value);
    Force kilonewtons(double value);

} // namespace units::force
