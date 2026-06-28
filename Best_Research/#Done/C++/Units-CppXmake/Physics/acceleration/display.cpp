#include "Physics/display/display.hpp"

#include <ostream>

namespace Physics::acceleration
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

    double value_in_unit(Acceleration value, AccelerationUnit unit)
    {
        switch (unit)
        {
        case AccelerationUnit::MetersPerSecondSquared:
            return value.to_meters_per_second_squared();
        case AccelerationUnit::StandardGravity:
            return value.to_standard_gravity();
        }

        return value.to_meters_per_second_squared();
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_meters_per_second_squared()
    {
        return display_as(AccelerationUnit::MetersPerSecondSquared);
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_standard_gravity()
    {
        return display_as(AccelerationUnit::StandardGravity);
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_meters_per_second_squared_precision(int precision)
    {
        return display_as_precision(AccelerationUnit::MetersPerSecondSquared, precision);
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_standard_gravity_precision(int precision)
    {
        return display_as_precision(AccelerationUnit::StandardGravity, precision);
    }

    std::ostream& operator<<(std::ostream& stream, Acceleration value)
    {
        return stream << value.display_as(AccelerationUnit::MetersPerSecondSquared);
    }

} // namespace Physics::acceleration
