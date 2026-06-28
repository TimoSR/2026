#pragma once

#include <optional>

#include "Physics/detail/quantity_display.hpp"
#include "Physics/time/time.hpp"
#include "Physics/velocity/velocity.hpp"

namespace Physics::length
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
            double _meters;

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

            std::optional<velocity::Velocity> checkedDivTime(time::Time time);
            std::optional<time::Time> checkedDivVelocity(velocity::Velocity velocity);

            friend bool operator==(Length left, Length right);

        private:
            explicit Length(double meters);
    };

    Length meters(double value);
    Length kilometers(double value);
    Length centimeters(double value);
    Length millimeters(double value);
    Length micrometers(double value);
    Length nanometers(double value);

} // namespace Physics::length
