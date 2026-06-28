#include "Physics/display/display.hpp"

#include <ostream>

namespace Physics::velocity
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

    double valueInUnit(Velocity value, VelocityUnit unit)
    {
        switch (unit)
        {
        case VelocityUnit::MetersPerSecond:
            return value.asMetersPerSecond();
        case VelocityUnit::KilometersPerHour:
            return value.asKilometersPerHour();
        }

        return value.asMetersPerSecond();
    }

    std::ostream& operator<<(std::ostream& stream, Velocity value)
    {
        return stream << value.displayAs(VelocityUnit::MetersPerSecond);
    }

} // namespace Physics::velocity
