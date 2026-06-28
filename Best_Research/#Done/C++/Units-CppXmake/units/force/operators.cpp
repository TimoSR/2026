#include "units/operations/operations.hpp"

namespace units::force
{

    Force operator+(Force left, Force right)
    {
        return Force::fromRawSi(left.rawSi() + right.rawSi());
    }

    Force operator-(Force left, Force right)
    {
        return Force::fromRawSi(left.rawSi() - right.rawSi());
    }

    Force operator-(Force value)
    {
        return Force::fromRawSi(-value.rawSi());
    }

    Force operator*(Force value, double scalar)
    {
        return Force::fromRawSi(value.rawSi() * scalar);
    }

    Force operator*(double scalar, Force value)
    {
        return Force::fromRawSi(scalar * value.rawSi());
    }

    Force operator/(Force value, double scalar)
    {
        return Force::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Force left, Force right)
    {
        return left.rawSi() / right.rawSi();
    }

} // namespace units::force
