#include "units/force/force.hpp"

#include "units/detail/math.hpp"

namespace units::force
{

    Force::Force(double newtons) : _newtons(newtons)
    {
    }

    Force Force::fromRawSi(double newtons)
    {
        return Force(newtons);
    }

    Force Force::newtons(double value)
    {
        return Force(value);
    }

    Force Force::millinewtons(double value)
    {
        return Force(value / 1'000.0);
    }

    Force Force::kilonewtons(double value)
    {
        return Force(value * 1'000.0);
    }

    double Force::rawSi()
    {
        return _newtons;
    }

    double Force::asNewtons()
    {
        return _newtons;
    }

    double Force::asMillinewtons()
    {
        return _newtons * 1'000.0;
    }

    double Force::asKilonewtons()
    {
        return _newtons / 1'000.0;
    }

    bool Force::approximatelyEquals(Force other, double epsilon)
    {
        return detail::absolute(_newtons - other._newtons) <= epsilon;
    }

    QuantityDisplay<Force, ForceUnit> Force::displayAs(ForceUnit unit)
    {
        return QuantityDisplay<Force, ForceUnit>(*this, unit);
    }

    QuantityDisplay<Force, ForceUnit> Force::displayAsPrecision(ForceUnit unit, int precision)
    {
        return QuantityDisplay<Force, ForceUnit>(*this, unit, precision);
    }

    bool operator==(Force left, Force right)
    {
        return left._newtons == right._newtons;
    }

} // namespace units::force
