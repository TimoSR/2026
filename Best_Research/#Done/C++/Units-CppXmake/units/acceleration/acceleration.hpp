#pragma once

#include "units/detail/quantity_display.hpp"

namespace units
{

    enum class AccelerationUnit
    {
        MetersPerSecondSquared,
        StandardGravity,
    };

    class Acceleration
    {
        public:
            static double standardGravityMetersPerSecondSquared();

            static Acceleration fromRawSi(double metersPerSecondSquared);

            static Acceleration metersPerSecondSquared(double value);

            static Acceleration standardGravity(double value);

            double rawSi();

            double asMetersPerSecondSquared();

            double asStandardGravity();

            bool approximatelyEquals(Acceleration other, double epsilon);

            QuantityDisplay<Acceleration, AccelerationUnit> displayAs(AccelerationUnit unit);

            QuantityDisplay<Acceleration, AccelerationUnit> displayAsPrecision(AccelerationUnit unit, int precision);

            friend bool operator==(Acceleration left, Acceleration right);

        private:
            explicit Acceleration(double metersPerSecondSquared);

            double metersPerSecondSquared_;
    };

} // namespace units
