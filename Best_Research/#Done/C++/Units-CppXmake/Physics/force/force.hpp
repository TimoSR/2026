#pragma once

#include <optional>

#include "physics/acceleration/acceleration.hpp"
#include "physics/detail/quantity_display.hpp"
#include "physics/mass/mass.hpp"

namespace physics::force
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
            static Force from_raw_si(double newtons);

            static Force newtons(double value);

            static std::optional<Force> try_newtons(double value);

            static Force millinewtons(double value);

            static std::optional<Force> try_millinewtons(double value);

            static Force kilonewtons(double value);

            static std::optional<Force> try_kilonewtons(double value);

            double raw_si();

            double to_newtons();

            double to_millinewtons();

            double to_kilonewtons();

            bool approximately_equals(Force other, double epsilon);

            QuantityDisplay<Force, ForceUnit> display_as(ForceUnit unit);

            QuantityDisplay<Force, ForceUnit> display_as_precision(ForceUnit unit, int precision);

            QuantityDisplay<Force, ForceUnit> display_newtons();
            QuantityDisplay<Force, ForceUnit> display_millinewtons();
            QuantityDisplay<Force, ForceUnit> display_kilonewtons();

            QuantityDisplay<Force, ForceUnit> display_newtons_precision(int precision);
            QuantityDisplay<Force, ForceUnit> display_millinewtons_precision(int precision);
            QuantityDisplay<Force, ForceUnit> display_kilonewtons_precision(int precision);

            std::optional<acceleration::Acceleration> checked_div_mass(mass::Mass mass);
            std::optional<mass::Mass> checked_div_acceleration(acceleration::Acceleration acceleration);

            friend bool operator==(Force left, Force right);

        private:
            explicit Force(double newtons);
    };

    Force newtons(double value);
    std::optional<Force> try_newtons(double value);
    Force force(double value);
    Force millinewtons(double value);
    std::optional<Force> try_millinewtons(double value);
    Force kilonewtons(double value);
    std::optional<Force> try_kilonewtons(double value);

} // namespace physics::force
