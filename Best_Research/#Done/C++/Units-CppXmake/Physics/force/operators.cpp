#include "physics/operations/operations.hpp"

namespace physics::force
{

    Force operator+(Force left, Force right)
    {
        return Force::from_raw_si(left.raw_si() + right.raw_si());
    }

    Force operator-(Force left, Force right)
    {
        return Force::from_raw_si(left.raw_si() - right.raw_si());
    }

    Force operator-(Force value)
    {
        return Force::from_raw_si(-value.raw_si());
    }

    Force operator*(Force value, double scalar)
    {
        return Force::from_raw_si(value.raw_si() * scalar);
    }

    Force operator*(double scalar, Force value)
    {
        return Force::from_raw_si(scalar * value.raw_si());
    }

    Force operator/(Force value, double scalar)
    {
        return Force::from_raw_si(value.raw_si() / scalar);
    }

    double operator/(Force left, Force right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace physics::force
