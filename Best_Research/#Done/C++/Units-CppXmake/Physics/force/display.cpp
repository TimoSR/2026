#include "Physics/display/display.hpp"

#include <ostream>

namespace Physics::force
{

    const char* symbol(ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return "N";
        case ForceUnit::Millinewtons:
            return "mN";
        case ForceUnit::Kilonewtons:
            return "kN";
        }

        return "";
    }

    double value_in_unit(Force value, ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return value.to_newtons();
        case ForceUnit::Millinewtons:
            return value.to_millinewtons();
        case ForceUnit::Kilonewtons:
            return value.to_kilonewtons();
        }

        return value.to_newtons();
    }

    QuantityDisplay<Force, ForceUnit> Force::display_newtons()
    {
        return display_as(ForceUnit::Newtons);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_millinewtons()
    {
        return display_as(ForceUnit::Millinewtons);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_kilonewtons()
    {
        return display_as(ForceUnit::Kilonewtons);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_newtons_precision(int precision)
    {
        return display_as_precision(ForceUnit::Newtons, precision);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_millinewtons_precision(int precision)
    {
        return display_as_precision(ForceUnit::Millinewtons, precision);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_kilonewtons_precision(int precision)
    {
        return display_as_precision(ForceUnit::Kilonewtons, precision);
    }

    std::ostream& operator<<(std::ostream& stream, Force value)
    {
        return stream << value.display_as(ForceUnit::Newtons);
    }

} // namespace Physics::force
