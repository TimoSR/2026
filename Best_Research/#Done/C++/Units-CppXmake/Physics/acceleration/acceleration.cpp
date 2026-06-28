#include "Physics/acceleration/acceleration.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::acceleration
{

    double Acceleration::standard_gravity_meters_per_second_squared()
    {
        return 9.80665;
    }

    Acceleration::Acceleration(double meters_per_second_squared) : _meters_per_second_squared(meters_per_second_squared)
    {
    }

    Acceleration Acceleration::from_raw_si(double meters_per_second_squared)
    {
        return Acceleration(meters_per_second_squared);
    }

    Acceleration Acceleration::meters_per_second_squared(double value)
    {
        return Acceleration(value);
    }

    Acceleration Acceleration::standard_gravity(double value)
    {
        return Acceleration(value * standard_gravity_meters_per_second_squared());
    }

    double Acceleration::raw_si()
    {
        return _meters_per_second_squared;
    }

    double Acceleration::to_meters_per_second_squared()
    {
        return _meters_per_second_squared;
    }

    double Acceleration::to_standard_gravity()
    {
        return _meters_per_second_squared / standard_gravity_meters_per_second_squared();
    }

    bool Acceleration::approximately_equals(Acceleration other, double epsilon)
    {
        return detail::absolute(_meters_per_second_squared - other._meters_per_second_squared) <= epsilon;
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_as(AccelerationUnit unit)
    {
        return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit);
    }

    QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::display_as_precision(AccelerationUnit unit, int precision)
    {
        return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit, precision);
    }

    bool operator==(Acceleration left, Acceleration right)
    {
        return left._meters_per_second_squared == right._meters_per_second_squared;
    }

} // namespace Physics::acceleration
