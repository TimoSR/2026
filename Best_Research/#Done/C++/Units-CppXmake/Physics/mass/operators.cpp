#include "Physics/operations/operations.hpp"

namespace Physics::mass
{

    Mass operator+(Mass left, Mass right)
    {
        return Mass::fromRawSi(left.rawSi() + right.rawSi());
    }

    Mass operator-(Mass left, Mass right)
    {
        return Mass::fromRawSi(left.rawSi() - right.rawSi());
    }

    Mass operator-(Mass value)
    {
        return Mass::fromRawSi(-value.rawSi());
    }

    Mass operator*(Mass value, double scalar)
    {
        return Mass::fromRawSi(value.rawSi() * scalar);
    }

    Mass operator*(double scalar, Mass value)
    {
        return Mass::fromRawSi(scalar * value.rawSi());
    }

    Mass operator/(Mass value, double scalar)
    {
        return Mass::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Mass left, Mass right)
    {
        return left.rawSi() / right.rawSi();
    }

} // namespace Physics::mass
