module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

namespace aztro::physics::length
{

    std::optional<Length> Length::try_meters(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Length::meters(value);
    }

    std::optional<Length> Length::try_kilometers(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Length::kilometers(value);
    }

    std::optional<Length> Length::try_centimeters(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Length::centimeters(value);
    }

    std::optional<Length> Length::try_millimeters(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Length::millimeters(value);
    }

    std::optional<Length> Length::try_micrometers(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Length::micrometers(value);
    }

    std::optional<Length> Length::try_nanometers(double value)
    {
        if (!std::isfinite(value))
        {
            return std::nullopt;
        }

        return Length::nanometers(value);
    }

    Length meters(double value)
    {
        return Length::meters(value);
    }

    std::optional<Length> try_meters(double value)
    {
        return Length::try_meters(value);
    }

    Length kilometers(double value)
    {
        return Length::kilometers(value);
    }

    std::optional<Length> try_kilometers(double value)
    {
        return Length::try_kilometers(value);
    }

    Length centimeters(double value)
    {
        return Length::centimeters(value);
    }

    std::optional<Length> try_centimeters(double value)
    {
        return Length::try_centimeters(value);
    }

    Length millimeters(double value)
    {
        return Length::millimeters(value);
    }

    std::optional<Length> try_millimeters(double value)
    {
        return Length::try_millimeters(value);
    }

    Length micrometers(double value)
    {
        return Length::micrometers(value);
    }

    std::optional<Length> try_micrometers(double value)
    {
        return Length::try_micrometers(value);
    }

    Length nanometers(double value)
    {
        return Length::nanometers(value);
    }

    std::optional<Length> try_nanometers(double value)
    {
        return Length::try_nanometers(value);
    }

} // namespace aztro::physics::length
