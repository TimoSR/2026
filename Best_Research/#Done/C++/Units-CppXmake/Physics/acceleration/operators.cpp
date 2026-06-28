#include "physics/operations/operations.hpp"

namespace physics::acceleration
{

    Acceleration operator+(Acceleration left, Acceleration right)
    {
        return Acceleration::from_raw_si(left.raw_si() + right.raw_si());
    }

    Acceleration operator-(Acceleration left, Acceleration right)
    {
        return Acceleration::from_raw_si(left.raw_si() - right.raw_si());
    }

    Acceleration operator-(Acceleration value)
    {
        return Acceleration::from_raw_si(-value.raw_si());
    }

    Acceleration operator*(Acceleration value, double scalar)
    {
        return Acceleration::from_raw_si(value.raw_si() * scalar);
    }

    Acceleration operator*(double scalar, Acceleration value)
    {
        return Acceleration::from_raw_si(scalar * value.raw_si());
    }

    Acceleration operator/(Acceleration value, double scalar)
    {
        return Acceleration::from_raw_si(value.raw_si() / scalar);
    }

    double operator/(Acceleration left, Acceleration right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace physics::acceleration
