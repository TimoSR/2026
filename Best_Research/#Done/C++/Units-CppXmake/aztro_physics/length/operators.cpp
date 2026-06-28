module aztro_physics;

namespace aztro::physics::length
{

    Length operator+(Length left, Length right)
    {
        return Length::from_raw_si(left.raw_si() + right.raw_si());
    }

    Length operator-(Length left, Length right)
    {
        return Length::from_raw_si(left.raw_si() - right.raw_si());
    }

    Length operator-(Length value)
    {
        return Length::from_raw_si(-value.raw_si());
    }

    Length operator*(Length value, double scalar)
    {
        return Length::from_raw_si(value.raw_si() * scalar);
    }

    Length operator*(double scalar, Length value)
    {
        return Length::from_raw_si(scalar * value.raw_si());
    }

    Length operator/(Length value, double scalar)
    {
        return Length::from_raw_si(value.raw_si() / scalar);
    }

    double operator/(Length left, Length right)
    {
        return left.raw_si() / right.raw_si();
    }

} // namespace aztro::physics::length
