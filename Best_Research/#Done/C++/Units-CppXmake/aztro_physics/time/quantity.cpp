module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

namespace aztro::physics::time
{

    std::optional<Time> Time::try_seconds(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Time::seconds(value);
    }

    std::optional<Time> Time::try_milliseconds(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Time::milliseconds(value);
    }

    std::optional<Time> Time::try_microseconds(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Time::microseconds(value);
    }

    std::optional<Time> Time::try_nanoseconds(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Time::nanoseconds(value);
    }

    std::optional<Time> Time::try_minutes(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Time::minutes(value);
    }

    std::optional<Time> Time::try_hours(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Time::hours(value);
    }

    Time seconds(double value)
    {
        return Time::seconds(value);
    }

    std::optional<Time> try_seconds(double value)
    {
        return Time::try_seconds(value);
    }

    Time milliseconds(double value)
    {
        return Time::milliseconds(value);
    }

    std::optional<Time> try_milliseconds(double value)
    {
        return Time::try_milliseconds(value);
    }

    Time microseconds(double value)
    {
        return Time::microseconds(value);
    }

    std::optional<Time> try_microseconds(double value)
    {
        return Time::try_microseconds(value);
    }

    Time nanoseconds(double value)
    {
        return Time::nanoseconds(value);
    }

    std::optional<Time> try_nanoseconds(double value)
    {
        return Time::try_nanoseconds(value);
    }

    Time minutes(double value)
    {
        return Time::minutes(value);
    }

    std::optional<Time> try_minutes(double value)
    {
        return Time::try_minutes(value);
    }

    Time hours(double value)
    {
        return Time::hours(value);
    }

    std::optional<Time> try_hours(double value)
    {
        return Time::try_hours(value);
    }

} // namespace aztro::physics::time
