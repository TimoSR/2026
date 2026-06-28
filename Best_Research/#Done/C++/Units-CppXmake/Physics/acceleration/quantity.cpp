#include "Physics/acceleration/acceleration.hpp"

#include <cmath>
#include <optional>

namespace Physics::acceleration
{

    std::optional<Acceleration> Acceleration::try_meters_per_second_squared(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Acceleration::meters_per_second_squared(value);
    }

    std::optional<Acceleration> Acceleration::try_standard_gravity(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Acceleration::standard_gravity(value);
    }

    Acceleration meters_per_second_squared(double value)
    {
        return Acceleration::meters_per_second_squared(value);
    }

    std::optional<Acceleration> try_meters_per_second_squared(double value)
    {
        return Acceleration::try_meters_per_second_squared(value);
    }

    Acceleration meters_pr_second_pr_second(double value)
    {
        return Acceleration::meters_per_second_squared(value);
    }

    Acceleration standard_gravity(double value)
    {
        return Acceleration::standard_gravity(value);
    }

    std::optional<Acceleration> try_standard_gravity(double value)
    {
        return Acceleration::try_standard_gravity(value);
    }

} // namespace Physics::acceleration
