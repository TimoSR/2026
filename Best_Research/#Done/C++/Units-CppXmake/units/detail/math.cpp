#include "units/detail/math.hpp"

namespace units::detail
{

    double absolute(double value)
    {
        return value < 0.0 ? -value : value;
    }

} // namespace units::detail
