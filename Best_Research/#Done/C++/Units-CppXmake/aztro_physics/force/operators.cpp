module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

namespace aztro::physics::force
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

} // namespace aztro::physics::force
