#include "units/constructors/constructors.hpp"

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
