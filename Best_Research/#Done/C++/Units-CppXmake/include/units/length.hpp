#pragma once

#include <optional>

#include "units/detail/quantity_display.hpp"
#include "units/forward.hpp"

namespace units
{

    enum class LengthUnit
    {
        Meters,
        Kilometers,
        Centimeters,
        Millimeters,
        Micrometers,
        Nanometers,
    };

    class Length
    {
        public:
            static Length fromRawSi(double meters);

            static Length meters(double value);

            static Length kilometers(double value);

            static Length centimeters(double value);

            static Length millimeters(double value);

            static Length micrometers(double value);

            static Length nanometers(double value);

            double rawSi();

            double asMeters();

            double asKilometers();

            double asCentimeters();

            double asMillimeters();

            double asMicrometers();

            double asNanometers();

            bool approximatelyEquals(Length other, double epsilon);

            QuantityDisplay<Length, LengthUnit> displayAs(LengthUnit unit);

            QuantityDisplay<Length, LengthUnit> displayAsPrecision(LengthUnit unit, int precision);

            std::optional<Velocity> checkedDivTime(Time time);
            std::optional<Time> checkedDivVelocity(Velocity velocity);

            friend bool operator==(Length left, Length right);

        private:
            explicit Length(double meters);

            double meters_;
    };

} // namespace units
