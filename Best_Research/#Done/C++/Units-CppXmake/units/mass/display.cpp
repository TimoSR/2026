#include "units/display/display.hpp"

#include <ostream>

namespace units::mass
{

    const char* symbol(MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return "kg";
        case MassUnit::Grams:
            return "g";
        case MassUnit::Milligrams:
            return "mg";
        case MassUnit::Micrograms:
            return "ug";
        case MassUnit::Tons:
            return "t";
        }

        return "";
    }

    double valueInUnit(Mass value, MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return value.asKilograms();
        case MassUnit::Grams:
            return value.asGrams();
        case MassUnit::Milligrams:
            return value.asMilligrams();
        case MassUnit::Micrograms:
            return value.asMicrograms();
        case MassUnit::Tons:
            return value.asTons();
        }

        return value.asKilograms();
    }

    std::ostream& operator<<(std::ostream& stream, Mass value)
    {
        return stream << value.displayAs(MassUnit::Kilograms);
    }

} // namespace units::mass
