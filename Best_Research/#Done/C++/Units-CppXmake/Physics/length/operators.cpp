#include "Physics/operations/operations.hpp"

namespace Physics::length
{

    Length operator+(Length left, Length right)
    {
        return Length::fromRawSi(left.rawSi() + right.rawSi());
    }

    Length operator-(Length left, Length right)
    {
        return Length::fromRawSi(left.rawSi() - right.rawSi());
    }

    Length operator-(Length value)
    {
        return Length::fromRawSi(-value.rawSi());
    }

    Length operator*(Length value, double scalar)
    {
        return Length::fromRawSi(value.rawSi() * scalar);
    }

    Length operator*(double scalar, Length value)
    {
        return Length::fromRawSi(scalar * value.rawSi());
    }

    Length operator/(Length value, double scalar)
    {
        return Length::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Length left, Length right)
    {
        return left.rawSi() / right.rawSi();
    }

} // namespace Physics::length
