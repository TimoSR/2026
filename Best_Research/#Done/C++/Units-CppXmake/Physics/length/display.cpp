#include "physics/display/display.hpp"

#include <ostream>

namespace physics::length
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

    double value_in_unit(Length value, LengthUnit unit)
    {
        switch (unit)
        {
        case LengthUnit::Meters:
            return value.to_meters();
        case LengthUnit::Kilometers:
            return value.to_kilometers();
        case LengthUnit::Centimeters:
            return value.to_centimeters();
        case LengthUnit::Millimeters:
            return value.to_millimeters();
        case LengthUnit::Micrometers:
            return value.to_micrometers();
        case LengthUnit::Nanometers:
            return value.to_nanometers();
        }

        return value.to_meters();
    }

    QuantityDisplay<Length, LengthUnit> Length::display_meters()
    {
        return display_as(LengthUnit::Meters);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_kilometers()
    {
        return display_as(LengthUnit::Kilometers);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_centimeters()
    {
        return display_as(LengthUnit::Centimeters);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_millimeters()
    {
        return display_as(LengthUnit::Millimeters);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_micrometers()
    {
        return display_as(LengthUnit::Micrometers);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_nanometers()
    {
        return display_as(LengthUnit::Nanometers);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_meters_precision(int precision)
    {
        return display_as_precision(LengthUnit::Meters, precision);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_kilometers_precision(int precision)
    {
        return display_as_precision(LengthUnit::Kilometers, precision);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_centimeters_precision(int precision)
    {
        return display_as_precision(LengthUnit::Centimeters, precision);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_millimeters_precision(int precision)
    {
        return display_as_precision(LengthUnit::Millimeters, precision);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_micrometers_precision(int precision)
    {
        return display_as_precision(LengthUnit::Micrometers, precision);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_nanometers_precision(int precision)
    {
        return display_as_precision(LengthUnit::Nanometers, precision);
    }

    std::ostream& operator<<(std::ostream& stream, Length value)
    {
        return stream << value.display_as(LengthUnit::Meters);
    }

} // namespace physics::length
