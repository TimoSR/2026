#include "Physics/detail/math.hpp"

namespace Physics::detail
{

    double absolute(double value)
    {
        return value < 0.0 ? -value : value;
    }

} // namespace Physics::detail
