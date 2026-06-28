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

    double valueInUnit(Force value, ForceUnit unit)
    {
        switch (unit)
        {
        case ForceUnit::Newtons:
            return value.asNewtons();
        case ForceUnit::Millinewtons:
            return value.asMillinewtons();
        case ForceUnit::Kilonewtons:
            return value.asKilonewtons();
        }

        return value.asNewtons();
    }

    std::ostream& operator<<(std::ostream& stream, Force value)
    {
        return stream << value.displayAs(ForceUnit::Newtons);
    }

} // namespace Physics::force
