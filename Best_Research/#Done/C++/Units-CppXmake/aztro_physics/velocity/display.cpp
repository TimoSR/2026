module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

namespace aztro::physics::velocity
{

    const char* symbol(VelocityUnit unit)
    {
        switch (unit)
        {
        case VelocityUnit::MetersPerSecond:
            return "m/s";
        case VelocityUnit::KilometersPerHour:
            return "km/h";
        }

        return "";
    }

    double value_in_unit(Velocity value, VelocityUnit unit)
    {
        switch (unit)
        {
        case VelocityUnit::MetersPerSecond:
            return value.to_meters_per_second();
        case VelocityUnit::KilometersPerHour:
            return value.to_kilometers_per_hour();
        }

        return value.to_meters_per_second();
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::display_meters_per_second()
    {
        return display_as(VelocityUnit::MetersPerSecond);
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::display_kilometers_per_hour()
    {
        return display_as(VelocityUnit::KilometersPerHour);
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::display_meters_per_second_precision(int precision)
    {
        return display_as_precision(VelocityUnit::MetersPerSecond, precision);
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::display_kilometers_per_hour_precision(int precision)
    {
        return display_as_precision(VelocityUnit::KilometersPerHour, precision);
    }

    std::ostream& operator<<(std::ostream& stream, Velocity value)
    {
        return stream << value.display_as(VelocityUnit::MetersPerSecond);
    }

} // namespace aztro::physics::velocity
