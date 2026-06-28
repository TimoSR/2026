#include "units/display.hpp"

#include <iomanip>
#include <ostream>

#include "units/acceleration.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units
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

    const char* symbol(TimeUnit unit)
    {
        switch (unit)
        {
        case TimeUnit::Seconds:
            return "s";
        case TimeUnit::Milliseconds:
            return "ms";
        case TimeUnit::Microseconds:
            return "us";
        case TimeUnit::Nanoseconds:
            return "ns";
        case TimeUnit::Minutes:
            return "min";
        case TimeUnit::Hours:
            return "h";
        }

        return "";
    }

    const char* symbol(MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return "kg";
        case MassUnit::Grams:
            return "g";
        case MassUnit::Milligrams:
            return "mg";
        case MassUnit::Micrograms:
            return "ug";
        case MassUnit::Tons:
            return "t";
        }

        return "";
    }

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

    const char* symbol(ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return "N";
        case ForceUnit::Millinewtons:
            return "mN";
        case ForceUnit::Kilonewtons:
            return "kN";
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

    double valueInUnit(Time value, TimeUnit unit)
    {
        switch (unit)
        {
        case TimeUnit::Seconds:
            return value.asSeconds();
        case TimeUnit::Milliseconds:
            return value.asMilliseconds();
        case TimeUnit::Microseconds:
            return value.asMicroseconds();
        case TimeUnit::Nanoseconds:
            return value.asNanoseconds();
        case TimeUnit::Minutes:
            return value.asMinutes();
        case TimeUnit::Hours:
            return value.asHours();
        }

        return value.asSeconds();
    }

    double valueInUnit(Mass value, MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return value.asKilograms();
        case MassUnit::Grams:
            return value.asGrams();
        case MassUnit::Milligrams:
            return value.asMilligrams();
        case MassUnit::Micrograms:
            return value.asMicrograms();
        case MassUnit::Tons:
            return value.asTons();
        }

        return value.asKilograms();
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

    double valueInUnit(Force value, ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return value.asNewtons();
        case ForceUnit::Millinewtons:
            return value.asMillinewtons();
        case ForceUnit::Kilonewtons:
            return value.asKilonewtons();
        }

        return value.asNewtons();
    }

    namespace
    {

        template <typename Quantity, typename Unit> std::ostream& writeDisplay(std::ostream& stream, QuantityDisplay<Quantity, Unit> display)
        {
            auto flags = stream.flags();
            auto precision = stream.precision();

            if (display.precision().has_value())
            {
                stream << std::fixed << std::setprecision(*display.precision());
            }

            stream << valueInUnit(display.value(), display.unit()) << ' ' << symbol(display.unit());

            stream.flags(flags);
            stream.precision(precision);
            return stream;
        }

    } // namespace

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Length, LengthUnit> display)
    {
        return writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Time, TimeUnit> display)
    {
        return writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Mass, MassUnit> display)
    {
        return writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Velocity, VelocityUnit> display)
    {
        return writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Acceleration, AccelerationUnit> display)
    {
        return writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<Force, ForceUnit> display)
    {
        return writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, Length value)
    {
        return stream << value.displayAs(LengthUnit::Meters);
    }

    std::ostream& operator<<(std::ostream& stream, Time value)
    {
        return stream << value.displayAs(TimeUnit::Seconds);
    }

    std::ostream& operator<<(std::ostream& stream, Mass value)
    {
        return stream << value.displayAs(MassUnit::Kilograms);
    }

    std::ostream& operator<<(std::ostream& stream, Velocity value)
    {
        return stream << value.displayAs(VelocityUnit::MetersPerSecond);
    }

    std::ostream& operator<<(std::ostream& stream, Acceleration value)
    {
        return stream << value.displayAs(AccelerationUnit::MetersPerSecondSquared);
    }

    std::ostream& operator<<(std::ostream& stream, Force value)
    {
        return stream << value.displayAs(ForceUnit::Newtons);
    }

} // namespace units
