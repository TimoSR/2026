#include "Physics/mass/mass.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::mass
{

    Mass::Mass(double kilograms) : _kilograms(kilograms)
    {
    }

    Mass Mass::fromRawSi(double kilograms)
    {
        return Mass(kilograms);
    }

    Mass Mass::kilograms(double value)
    {
        return Mass(value);
    }

    Mass Mass::kilogram(double value)
    {
        return kilograms(value);
    }

    Mass Mass::grams(double value)
    {
        return Mass(value / 1'000.0);
    }

    Mass Mass::milligrams(double value)
    {
        return Mass(value / 1'000'000.0);
    }

    Mass Mass::micrograms(double value)
    {
        return Mass(value / 1'000'000'000.0);
    }

    Mass Mass::tons(double value)
    {
        return Mass(value * 1'000.0);
    }

    double Mass::rawSi()
    {
        return _kilograms;
    }

    double Mass::asKilograms()
    {
        return _kilograms;
    }

    double Mass::asGrams()
    {
        return _kilograms * 1'000.0;
    }

    double Mass::asMilligrams()
    {
        return _kilograms * 1'000'000.0;
    }

    double Mass::asMicrograms()
    {
        return _kilograms * 1'000'000'000.0;
    }

    double Mass::asTons()
    {
        return _kilograms / 1'000.0;
    }

    bool Mass::approximatelyEquals(Mass other, double epsilon)
    {
        return detail::absolute(_kilograms - other._kilograms) <= epsilon;
    }

    QuantityDisplay<Mass, MassUnit> Mass::displayAs(MassUnit unit)
    {
        return QuantityDisplay<Mass, MassUnit>(*this, unit);
    }

    QuantityDisplay<Mass, MassUnit> Mass::displayAsPrecision(MassUnit unit, int precision)
    {
        return QuantityDisplay<Mass, MassUnit>(*this, unit, precision);
    }

    bool operator==(Mass left, Mass right)
    {
        return left._kilograms == right._kilograms;
    }

} // namespace Physics::mass
