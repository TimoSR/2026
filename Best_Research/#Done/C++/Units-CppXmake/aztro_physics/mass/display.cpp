module;

#include <ostream>

module aztro_physics;

namespace aztro::physics::mass
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

    double value_in_unit(Mass value, MassUnit unit)
    {
        switch (unit)
        {
        case MassUnit::Kilograms:
            return value.to_kilograms();
        case MassUnit::Grams:
            return value.to_grams();
        case MassUnit::Milligrams:
            return value.to_milligrams();
        case MassUnit::Micrograms:
            return value.to_micrograms();
        case MassUnit::Tons:
            return value.to_tons();
        }

        return value.to_kilograms();
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_kilograms()
    {
        return display_as(MassUnit::Kilograms);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_grams()
    {
        return display_as(MassUnit::Grams);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_milligrams()
    {
        return display_as(MassUnit::Milligrams);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_micrograms()
    {
        return display_as(MassUnit::Micrograms);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_tons()
    {
        return display_as(MassUnit::Tons);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_kilograms_precision(int precision)
    {
        return display_as_precision(MassUnit::Kilograms, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_grams_precision(int precision)
    {
        return display_as_precision(MassUnit::Grams, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_milligrams_precision(int precision)
    {
        return display_as_precision(MassUnit::Milligrams, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_micrograms_precision(int precision)
    {
        return display_as_precision(MassUnit::Micrograms, precision);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_tons_precision(int precision)
    {
        return display_as_precision(MassUnit::Tons, precision);
    }

    std::ostream& operator<<(std::ostream& stream, Mass value)
    {
        return stream << value.display_as(MassUnit::Kilograms);
    }

} // namespace aztro::physics::mass
