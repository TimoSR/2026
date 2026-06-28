#include "units/constructors/constructors.hpp"

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
