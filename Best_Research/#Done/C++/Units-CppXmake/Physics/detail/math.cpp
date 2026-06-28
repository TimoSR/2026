#include "physics/detail/math.hpp"

namespace physics::detail
{

    double absolute(double value)
    {
        return value < 0.0 ? -value : value;
    }

} // namespace physics::detail
