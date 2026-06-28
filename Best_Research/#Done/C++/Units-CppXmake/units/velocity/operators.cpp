#include "units/operations/operations.hpp"

namespace units::velocity
{

    Velocity operator+(Velocity left, Velocity right)
    {
        return Velocity::fromRawSi(left.rawSi() + right.rawSi());
    }

    Velocity operator-(Velocity left, Velocity right)
    {
        return Velocity::fromRawSi(left.rawSi() - right.rawSi());
    }

    Velocity operator-(Velocity value)
    {
        return Velocity::fromRawSi(-value.rawSi());
    }

    Velocity operator*(Velocity value, double scalar)
    {
        return Velocity::fromRawSi(value.rawSi() * scalar);
    }

    Velocity operator*(double scalar, Velocity value)
    {
        return Velocity::fromRawSi(scalar * value.rawSi());
    }

    Velocity operator/(Velocity value, double scalar)
    {
        return Velocity::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Velocity left, Velocity right)
    {
        return left.rawSi() / right.rawSi();
    }

} // namespace units::velocity
