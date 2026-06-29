#pragma once

#include <optional>

#include "physics/detail/quantity_display.hpp"

namespace physics::mass
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
        private:
            double _kilograms;

        private:
            explicit Mass(double kilograms);

        public:
            static Mass from_raw_si(double kilograms);

            static Mass kilograms(double value);

            static std::optional<Mass> try_kilograms(double value);

            static Mass kilogram(double value);

            static std::optional<Mass> try_kilogram(double value);

            static Mass grams(double value);

            static std::optional<Mass> try_grams(double value);

            static Mass milligrams(double value);

            static std::optional<Mass> try_milligrams(double value);

            static Mass micrograms(double value);

            static std::optional<Mass> try_micrograms(double value);

            static Mass tons(double value);

            static std::optional<Mass> try_tons(double value);

            double raw_si();

            double to_kilograms();

            double to_grams();

            double to_milligrams();

            double to_micrograms();

            double to_tons();

            bool approximately_equals(Mass other, double epsilon);

            QuantityDisplay<Mass, MassUnit> display_as(MassUnit unit);

            QuantityDisplay<Mass, MassUnit> display_as_precision(MassUnit unit, int precision);

            QuantityDisplay<Mass, MassUnit> display_kilograms();
            QuantityDisplay<Mass, MassUnit> display_grams();
            QuantityDisplay<Mass, MassUnit> display_milligrams();
            QuantityDisplay<Mass, MassUnit> display_micrograms();
            QuantityDisplay<Mass, MassUnit> display_tons();

            QuantityDisplay<Mass, MassUnit> display_kilograms_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_grams_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_milligrams_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_micrograms_precision(int precision);
            QuantityDisplay<Mass, MassUnit> display_tons_precision(int precision);

            friend bool operator==(Mass left, Mass right);
    };

    Mass kilograms(double value);
    std::optional<Mass> try_kilograms(double value);
    Mass kilogram(double value);
    std::optional<Mass> try_kilogram(double value);
    Mass grams(double value);
    std::optional<Mass> try_grams(double value);
    Mass milligrams(double value);
    std::optional<Mass> try_milligrams(double value);
    Mass micrograms(double value);
    std::optional<Mass> try_micrograms(double value);
    Mass tons(double value);
    std::optional<Mass> try_tons(double value);

} // namespace physics::mass
