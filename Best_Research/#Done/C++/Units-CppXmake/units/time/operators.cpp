#include "units/operations/operations.hpp"

namespace units::time
{

    Time operator+(Time left, Time right)
    {
        return Time::fromRawSi(left.rawSi() + right.rawSi());
    }

    Time operator-(Time left, Time right)
    {
        return Time::fromRawSi(left.rawSi() - right.rawSi());
    }

    Time operator-(Time value)
    {
        return Time::fromRawSi(-value.rawSi());
    }

    Time operator*(Time value, double scalar)
    {
        return Time::fromRawSi(value.rawSi() * scalar);
    }

    Time operator*(double scalar, Time value)
    {
        return Time::fromRawSi(scalar * value.rawSi());
    }

    Time operator/(Time value, double scalar)
    {
        return Time::fromRawSi(value.rawSi() / scalar);
    }

    double operator/(Time left, Time right)
    {
        return left.rawSi() / right.rawSi();
    }

} // namespace units::time
