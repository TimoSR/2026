#include "units/operations/operations.hpp"

namespace units::acceleration
{

    Acceleration operator+(Acceleration left, Acceleration right)
    {
        return Acceleration::fromRawSi(left.rawSi() + right.rawSi());
    }

    Acceleration operator-(Acceleration left, Acceleration right)
    {
        return Acceleration::fromRawSi(left.rawSi() - right.rawSi());
    }

    Acceleration operator-(Acceleration value)
    {
        return Acceleration::fromRawSi(-value.rawSi());
    }

    Acceleration operator*(Acceleration value, double scalar)
    {
        return Acceleration::fromRawSi(value.rawSi() * scalar);
    }

    Acceleration operator*(double scalar, Acceleration value)
    {
        return Acceleration::fromRawSi(scalar * value.rawSi());
    }

    Acceleration operator/(Acceleration value, double scalar)
    {
        return Acceleration::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Acceleration left, Acceleration right)
    {
        return left.rawSi() / right.rawSi();
    }

} // namespace units::acceleration
