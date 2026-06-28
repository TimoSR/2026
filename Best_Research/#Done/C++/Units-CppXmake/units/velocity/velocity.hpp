#pragma once

#include <optional>

#include "units/acceleration/acceleration.hpp"
#include "units/detail/quantity_display.hpp"
#include "units/time/time.hpp"

namespace units::velocity
{

    enum class VelocityUnit
    {
        MetersPerSecond,
        KilometersPerHour,
    };

    class Velocity
    {
            double _metersPerSecond;

        public:
            static Velocity fromRawSi(double metersPerSecond);

            static Velocity metersPerSecond(double value);

            static Velocity kilometersPerHour(double value);

            double rawSi();

            double asMetersPerSecond();

            double asKilometersPerHour();

            bool approximatelyEquals(Velocity other, double epsilon);

            QuantityDisplay<Velocity, VelocityUnit> displayAs(VelocityUnit unit);

            QuantityDisplay<Velocity, VelocityUnit> displayAsPrecision(VelocityUnit unit, int precision);

            std::optional<acceleration::Acceleration> checkedDivTime(time::Time time);
            std::optional<time::Time> checkedDivAcceleration(acceleration::Acceleration acceleration);

            friend bool operator==(Velocity left, Velocity right);

        private:
            explicit Velocity(double metersPerSecond);
    };

} // namespace units::velocity
