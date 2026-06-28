#include "Physics/mass/mass.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::mass
{

    Mass::Mass(double kilograms) : _kilograms(kilograms)
    {
    }

    Mass Mass::from_raw_si(double kilograms)
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

    double Mass::raw_si()
    {
        return _kilograms;
    }

    double Mass::to_kilograms()
    {
        return _kilograms;
    }

    double Mass::to_grams()
    {
        return _kilograms * 1'000.0;
    }

    double Mass::to_milligrams()
    {
        return _kilograms * 1'000'000.0;
    }

    double Mass::to_micrograms()
    {
        return _kilograms * 1'000'000'000.0;
    }

    double Mass::to_tons()
    {
        return _kilograms / 1'000.0;
    }

    bool Mass::approximately_equals(Mass other, double epsilon)
    {
        return detail::absolute(_kilograms - other._kilograms) <= epsilon;
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_as(MassUnit unit)
    {
        return QuantityDisplay<Mass, MassUnit>(*this, unit);
    }

    QuantityDisplay<Mass, MassUnit> Mass::display_as_precision(MassUnit unit, int precision)
    {
        return QuantityDisplay<Mass, MassUnit>(*this, unit, precision);
    }

    bool operator==(Mass left, Mass right)
    {
        return left._kilograms == right._kilograms;
    }

} // namespace Physics::mass
