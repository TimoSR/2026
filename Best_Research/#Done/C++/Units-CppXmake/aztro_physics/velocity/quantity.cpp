module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro.physics;

namespace aztro::physics::velocity
{

    std::optional<Velocity> Velocity::try_meters_per_second(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Velocity::meters_per_second(value);
    }

    std::optional<Velocity> Velocity::try_kilometers_per_hour(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Velocity::kilometers_per_hour(value);
    }

    Velocity meters_per_second(double value)
    {
        return Velocity::meters_per_second(value);
    }

    std::optional<Velocity> try_meters_per_second(double value)
    {
        return Velocity::try_meters_per_second(value);
    }

    Velocity meters_pr_second(double value)
    {
        return Velocity::meters_per_second(value);
    }

    Velocity kilometers_per_hour(double value)
    {
        return Velocity::kilometers_per_hour(value);
    }

    std::optional<Velocity> try_kilometers_per_hour(double value)
    {
        return Velocity::try_kilometers_per_hour(value);
    }

} // namespace aztro::physics::velocity
