#include "units/constructors/constructors.hpp"

namespace units::length
{

    Length meters(double value)
    {
        return Length::meters(value);
    }

    Length kilometers(double value)
    {
        return Length::kilometers(value);
    }

    Length centimeters(double value)
    {
        return Length::centimeters(value);
    }

    Length millimeters(double value)
    {
        return Length::millimeters(value);
    }

    Length micrometers(double value)
    {
        return Length::micrometers(value);
    }

    Length nanometers(double value)
    {
        return Length::nanometers(value);
    }

} // namespace units::length

namespace units::time
{

    Time seconds(double value)
    {
        return Time::seconds(value);
    }

    Time milliseconds(double value)
    {
        return Time::milliseconds(value);
    }

    Time microseconds(double value)
    {
        return Time::microseconds(value);
    }

    Time nanoseconds(double value)
    {
        return Time::nanoseconds(value);
    }

    Time minutes(double value)
    {
        return Time::minutes(value);
    }

    Time hours(double value)
    {
        return Time::hours(value);
    }

} // namespace units::time

namespace units::mass
{

    Mass kilograms(double value)
    {
        return Mass::kilograms(value);
    }

    Mass kilogram(double value)
    {
        return Mass::kilogram(value);
    }

    Mass grams(double value)
    {
        return Mass::grams(value);
    }

    Mass milligrams(double value)
    {
        return Mass::milligrams(value);
    }

    Mass micrograms(double value)
    {
        return Mass::micrograms(value);
    }

    Mass tons(double value)
    {
        return Mass::tons(value);
    }

} // namespace units::mass

namespace units::velocity
{

    Velocity metersPerSecond(double value)
    {
        return Velocity::metersPerSecond(value);
    }

    Velocity meters_pr_second(double value)
    {
        return Velocity::metersPerSecond(value);
    }

    Velocity kilometersPerHour(double value)
    {
        return Velocity::kilometersPerHour(value);
    }

} // namespace units::velocity

namespace units::acceleration
{

    Acceleration metersPerSecondSquared(double value)
    {
        return Acceleration::metersPerSecondSquared(value);
    }

    Acceleration meters_pr_second_pr_second(double value)
    {
        return Acceleration::metersPerSecondSquared(value);
    }

    Acceleration standardGravity(double value)
    {
        return Acceleration::standardGravity(value);
    }

} // namespace units::acceleration

namespace units::force
{

    Force newtons(double value)
    {
        return Force::newtons(value);
    }

    Force force(double value)
    {
        return Force::newtons(value);
    }

    Force millinewtons(double value)
    {
        return Force::millinewtons(value);
    }

    Force kilonewtons(double value)
    {
        return Force::kilonewtons(value);
    }

} // namespace units::force
