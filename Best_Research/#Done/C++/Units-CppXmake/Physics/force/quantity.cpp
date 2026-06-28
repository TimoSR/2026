#include "Physics/force/force.hpp"

#include <cmath>
#include <optional>

namespace Physics::force
{

    std::optional<Force> Force::try_newtons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Force::newtons(value);
    }

    std::optional<Force> Force::try_millinewtons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Force::millinewtons(value);
    }

    std::optional<Force> Force::try_kilonewtons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Force::kilonewtons(value);
    }

    Force newtons(double value)
    {
        return Force::newtons(value);
    }

    std::optional<Force> try_newtons(double value)
    {
        return Force::try_newtons(value);
    }

    Force force(double value)
    {
        return Force::newtons(value);
    }

    Force millinewtons(double value)
    {
        return Force::millinewtons(value);
    }

    std::optional<Force> try_millinewtons(double value)
    {
        return Force::try_millinewtons(value);
    }

    Force kilonewtons(double value)
    {
        return Force::kilonewtons(value);
    }

    std::optional<Force> try_kilonewtons(double value)
    {
        return Force::try_kilonewtons(value);
    }

} // namespace Physics::force
