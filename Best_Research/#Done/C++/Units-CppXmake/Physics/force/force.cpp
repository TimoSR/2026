#include "Physics/force/force.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::force
{

    Force::Force(double newtons) : _newtons(newtons)
    {
    }

    Force Force::from_raw_si(double newtons)
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

    double Force::raw_si()
    {
        return _newtons;
    }

    double Force::to_newtons()
    {
        return _newtons;
    }

    double Force::to_millinewtons()
    {
        return _newtons * 1'000.0;
    }

    double Force::to_kilonewtons()
    {
        return _newtons / 1'000.0;
    }

    bool Force::approximately_equals(Force other, double epsilon)
    {
        return detail::absolute(_newtons - other._newtons) <= epsilon;
    }

    QuantityDisplay<Force, ForceUnit> Force::display_as(ForceUnit unit)
    {
        return QuantityDisplay<Force, ForceUnit>(*this, unit);
    }

    QuantityDisplay<Force, ForceUnit> Force::display_as_precision(ForceUnit unit, int precision)
    {
        return QuantityDisplay<Force, ForceUnit>(*this, unit, precision);
    }

    bool operator==(Force left, Force right)
    {
        return left._newtons == right._newtons;
    }

} // namespace Physics::force
