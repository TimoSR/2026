#include "Physics/display/display.hpp"

#include <ostream>

namespace Physics::length
{

    const char* symbol(LengthUnit unit)
    {
        switch (unit)
        {
        case LengthUnit::Meters:
            return "m";
        case LengthUnit::Kilometers:
            return "km";
        case LengthUnit::Centimeters:
            return "cm";
        case LengthUnit::Millimeters:
            return "mm";
        case LengthUnit::Micrometers:
            return "um";
        case LengthUnit::Nanometers:
            return "nm";
        }

        return "";
    }

    double valueInUnit(Length value, LengthUnit unit)
    {
        switch (unit)
        {
        case LengthUnit::Meters:
            return value.asMeters();
        case LengthUnit::Kilometers:
            return value.asKilometers();
        case LengthUnit::Centimeters:
            return value.asCentimeters();
        case LengthUnit::Millimeters:
            return value.asMillimeters();
        case LengthUnit::Micrometers:
            return value.asMicrometers();
        case LengthUnit::Nanometers:
            return value.asNanometers();
        }

        return value.asMeters();
    }

    std::ostream& operator<<(std::ostream& stream, Length value)
    {
        return stream << value.displayAs(LengthUnit::Meters);
    }

} // namespace Physics::length
