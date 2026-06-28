#include "units/display/display.hpp"

#include <ostream>

namespace units::acceleration
{

    const char* symbol(AccelerationUnit unit)
    {
        switch (unit)
        {
        case AccelerationUnit::MetersPerSecondSquared:
            return "m/s^2";
        case AccelerationUnit::StandardGravity:
            return "g0";
        }

        return "";
    }

    double valueInUnit(Acceleration value, AccelerationUnit unit)
    {
        switch (unit)
        {
        case AccelerationUnit::MetersPerSecondSquared:
            return value.asMetersPerSecondSquared();
        case AccelerationUnit::StandardGravity:
            return value.asStandardGravity();
        }

        return value.asMetersPerSecondSquared();
    }

    std::ostream& operator<<(std::ostream& stream, Acceleration value)
    {
        return stream << value.displayAs(AccelerationUnit::MetersPerSecondSquared);
    }

} // namespace units::acceleration
