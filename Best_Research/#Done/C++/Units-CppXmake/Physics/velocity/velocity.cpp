#include "physics/velocity/velocity.hpp"

#include "physics/detail/math.hpp"

namespace physics::velocity
{

    Velocity::Velocity(double meters_per_second) : _meters_per_second(meters_per_second)
    {
    }

    Velocity Velocity::from_raw_si(double meters_per_second)
    {
        return Velocity(meters_per_second);
    }

    Velocity Velocity::meters_per_second(double value)
    {
        return Velocity(value);
    }

    Velocity Velocity::kilometers_per_hour(double value)
    {
        return Velocity(value / 3.6);
    }

    double Velocity::raw_si()
    {
        return _meters_per_second;
    }

    double Velocity::to_meters_per_second()
    {
        return _meters_per_second;
    }

    double Velocity::to_kilometers_per_hour()
    {
        return _meters_per_second * 3.6;
    }

    bool Velocity::approximately_equals(Velocity other, double epsilon)
    {
        return detail::absolute(_meters_per_second - other._meters_per_second) <= epsilon;
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::display_as(VelocityUnit unit)
    {
        return QuantityDisplay<Velocity, VelocityUnit>(*this, unit);
    }

    QuantityDisplay<Velocity, VelocityUnit> Velocity::display_as_precision(VelocityUnit unit, int precision)
    {
        return QuantityDisplay<Velocity, VelocityUnit>(*this, unit, precision);
    }

    bool operator==(Velocity left, Velocity right)
    {
        return left._meters_per_second == right._meters_per_second;
    }

} // namespace physics::velocity
