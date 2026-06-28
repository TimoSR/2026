#include "units/constructors/constructors.hpp"

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
