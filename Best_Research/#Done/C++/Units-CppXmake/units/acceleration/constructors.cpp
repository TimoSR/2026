#include "units/constructors/constructors.hpp"

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
