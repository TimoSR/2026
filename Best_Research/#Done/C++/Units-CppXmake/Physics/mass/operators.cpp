#include "Physics/operations/operations.hpp"

namespace Physics::mass
{

    Mass operator+(Mass left, Mass right)
    {
        return Mass::from_raw_si(left.raw_si() + right.raw_si());
    }

    Mass operator-(Mass left, Mass right)
    {
        return Mass::from_raw_si(left.raw_si() - right.raw_si());
    }

    Mass operator-(Mass value)
    {
        return Mass::from_raw_si(-value.raw_si());
    }

    Mass operator*(Mass value, double scalar)
    {
        return Mass::from_raw_si(value.raw_si() * scalar);
    }

    Mass operator*(double scalar, Mass value)
    {
        return Mass::from_raw_si(scalar * value.raw_si());
    }

    Mass operator/(Mass value, double scalar)
    {
        return Mass::from_raw_si(value.raw_si() / scalar);
    }

    double operator/(Mass left, Mass right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace Physics::mass
