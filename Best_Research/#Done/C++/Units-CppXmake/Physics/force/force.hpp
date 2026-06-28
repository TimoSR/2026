#pragma once

#include <optional>

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/detail/quantity_display.hpp"
#include "Physics/mass/mass.hpp"

namespace Physics::force
{

    enum class ForceUnit
    {
        Newtons,
        Millinewtons,
        Kilonewtons,
    };

    class Force
    {
            double _newtons;

        public:
            static Force fromRawSi(double newtons);

            static Force newtons(double value);

            static Force millinewtons(double value);

            static Force kilonewtons(double value);

            double rawSi();

            double asNewtons();

            double asMillinewtons();

            double asKilonewtons();

            bool approximatelyEquals(Force other, double epsilon);

            QuantityDisplay<Force, ForceUnit> displayAs(ForceUnit unit);

            QuantityDisplay<Force, ForceUnit> displayAsPrecision(ForceUnit unit, int precision);

            std::optional<acceleration::Acceleration> checkedDivMass(mass::Mass mass);
            std::optional<mass::Mass> checkedDivAcceleration(acceleration::Acceleration acceleration);

            friend bool operator==(Force left, Force right);

        private:
            explicit Force(double newtons);
    };

    Force newtons(double value);
    Force force(double value);
    Force millinewtons(double value);
    Force kilonewtons(double value);

} // namespace Physics::force
