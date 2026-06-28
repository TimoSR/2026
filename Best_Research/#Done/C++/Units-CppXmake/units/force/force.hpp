#pragma once

#include <optional>

#include "units/acceleration/acceleration.hpp"
#include "units/detail/quantity_display.hpp"
#include "units/mass/mass.hpp"

namespace units
{

    enum class ForceUnit
    {
        Newtons,
        Millinewtons,
        Kilonewtons,
    };

    class Force
    {
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

            std::optional<Acceleration> checkedDivMass(Mass mass);
            std::optional<Mass> checkedDivAcceleration(Acceleration acceleration);

            friend bool operator==(Force left, Force right);

        private:
            explicit Force(double newtons);

            double newtons_;
    };

} // namespace units
