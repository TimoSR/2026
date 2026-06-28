#include "units/force.hpp"

#include "units/detail/math.hpp"

namespace units
{

    Force::Force(double newtons) : newtons_(newtons)
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
        return newtons_;
    }

    double Force::asNewtons()
    {
        return newtons_;
    }

    double Force::asMillinewtons()
    {
        return newtons_ * 1'000.0;
    }

    double Force::asKilonewtons()
    {
        return newtons_ / 1'000.0;
    }

    bool Force::approximatelyEquals(Force other, double epsilon)
    {
        return detail::absolute(newtons_ - other.newtons_) <= epsilon;
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
        return left.newtons_ == right.newtons_;
    }

} // namespace units
