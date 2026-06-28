#pragma once

#include <optional>

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/detail/quantity_display.hpp"
#include "Physics/time/time.hpp"

namespace Physics::velocity
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

    Velocity metersPerSecond(double value);
    Velocity meters_pr_second(double value);
    Velocity kilometersPerHour(double value);

} // namespace Physics::velocity
