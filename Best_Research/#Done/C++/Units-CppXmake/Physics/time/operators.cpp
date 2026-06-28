#include "physics/operations/operations.hpp"

namespace physics::time
{

    Time operator+(Time left, Time right)
    {
        return Time::from_raw_si(left.raw_si() + right.raw_si());
    }

    Time operator-(Time left, Time right)
    {
        return Time::from_raw_si(left.raw_si() - right.raw_si());
    }

    Time operator-(Time value)
    {
        return Time::from_raw_si(-value.raw_si());
    }

    Time operator*(Time value, double scalar)
    {
        return Time::from_raw_si(value.raw_si() * scalar);
    }

    Time operator*(double scalar, Time value)
    {
        return Time::from_raw_si(scalar * value.raw_si());
    }

    Time operator/(Time value, double scalar)
    {
        return Time::from_raw_si(value.raw_si() / scalar);
    }

    double operator/(Time left, Time right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace physics::time
