#include "Physics/force/force.hpp"

namespace Physics::force
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

} // namespace Physics::force
