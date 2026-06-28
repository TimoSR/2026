#pragma once

#include "Physics/detail/quantity_display.hpp"

namespace Physics::mass
{

    enum class MassUnit
    {
        Kilograms,
        Grams,
        Milligrams,
        Micrograms,
        Tons,
    };

    class Mass
    {
            double _kilograms;

        public:
            static Mass fromRawSi(double kilograms);

            static Mass kilograms(double value);

            static Mass kilogram(double value);

            static Mass grams(double value);

            static Mass milligrams(double value);

            static Mass micrograms(double value);

            static Mass tons(double value);

            double rawSi();

            double asKilograms();

            double asGrams();

            double asMilligrams();

            double asMicrograms();

            double asTons();

            bool approximatelyEquals(Mass other, double epsilon);

            QuantityDisplay<Mass, MassUnit> displayAs(MassUnit unit);

            QuantityDisplay<Mass, MassUnit> displayAsPrecision(MassUnit unit, int precision);

            friend bool operator==(Mass left, Mass right);

        private:
            explicit Mass(double kilograms);
    };

    Mass kilograms(double value);
    Mass kilogram(double value);
    Mass grams(double value);
    Mass milligrams(double value);
    Mass micrograms(double value);
    Mass tons(double value);

} // namespace Physics::mass
