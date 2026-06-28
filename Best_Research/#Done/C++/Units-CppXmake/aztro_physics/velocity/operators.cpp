module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro.physics;

namespace aztro::physics::velocity
{

    Velocity operator+(Velocity left, Velocity right)
    {
        return Velocity::from_raw_si(left.raw_si() + right.raw_si());
    }

    Velocity operator-(Velocity left, Velocity right)
    {
        return Velocity::from_raw_si(left.raw_si() - right.raw_si());
    }

    Velocity operator-(Velocity value)
    {
        return Velocity::from_raw_si(-value.raw_si());
    }

    Velocity operator*(Velocity value, double scalar)
    {
        return Velocity::from_raw_si(value.raw_si() * scalar);
    }

    Velocity operator*(double scalar, Velocity value)
    {
        return Velocity::from_raw_si(scalar * value.raw_si());
    }

    Velocity operator/(Velocity value, double scalar)
    {
        return Velocity::from_raw_si(value.raw_si() / scalar);
    }

    double operator/(Velocity left, Velocity right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace aztro::physics::velocity
