module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro.physics;

namespace aztro::physics::mass
{

    std::optional<Mass> Mass::try_kilograms(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::kilograms(value);
    }

    std::optional<Mass> Mass::try_kilogram(double value)
    {
        return Mass::try_kilograms(value);
    }

    std::optional<Mass> Mass::try_grams(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::grams(value);
    }

    std::optional<Mass> Mass::try_milligrams(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::milligrams(value);
    }

    std::optional<Mass> Mass::try_micrograms(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::micrograms(value);
    }

    std::optional<Mass> Mass::try_tons(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Mass::tons(value);
    }

    Mass kilograms(double value)
    {
        return Mass::kilograms(value);
    }

    std::optional<Mass> try_kilograms(double value)
    {
        return Mass::try_kilograms(value);
    }

    Mass kilogram(double value)
    {
        return Mass::kilogram(value);
    }

    std::optional<Mass> try_kilogram(double value)
    {
        return Mass::try_kilogram(value);
    }

    Mass grams(double value)
    {
        return Mass::grams(value);
    }

    std::optional<Mass> try_grams(double value)
    {
        return Mass::try_grams(value);
    }

    Mass milligrams(double value)
    {
        return Mass::milligrams(value);
    }

    std::optional<Mass> try_milligrams(double value)
    {
        return Mass::try_milligrams(value);
    }

    Mass micrograms(double value)
    {
        return Mass::micrograms(value);
    }

    std::optional<Mass> try_micrograms(double value)
    {
        return Mass::try_micrograms(value);
    }

    Mass tons(double value)
    {
        return Mass::tons(value);
    }

    std::optional<Mass> try_tons(double value)
    {
        return Mass::try_tons(value);
    }

} // namespace aztro::physics::mass
