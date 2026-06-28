module aztro_physics;

namespace aztro::physics::length
{

    Length::Length(double meters) : _meters(meters)
    {
    }

    Length Length::from_raw_si(double meters)
    {
        return Length(meters);
    }

    Length Length::meters(double value)
    {
        return Length(value);
    }

    Length Length::kilometers(double value)
    {
        return Length(value * 1'000.0);
    }

    Length Length::centimeters(double value)
    {
        return Length(value / 100.0);
    }

    Length Length::millimeters(double value)
    {
        return Length(value / 1'000.0);
    }

    Length Length::micrometers(double value)
    {
        return Length(value / 1'000'000.0);
    }

    Length Length::nanometers(double value)
    {
        return Length(value / 1'000'000'000.0);
    }

    double Length::raw_si()
    {
        return _meters;
    }

    double Length::to_meters()
    {
        return _meters;
    }

    double Length::to_kilometers()
    {
        return _meters / 1'000.0;
    }

    double Length::to_centimeters()
    {
        return _meters * 100.0;
    }

    double Length::to_millimeters()
    {
        return _meters * 1'000.0;
    }

    double Length::to_micrometers()
    {
        return _meters * 1'000'000.0;
    }

    double Length::to_nanometers()
    {
        return _meters * 1'000'000'000.0;
    }

    bool Length::approximately_equals(Length other, double epsilon)
    {
        return detail::absolute(_meters - other._meters) <= epsilon;
    }

    QuantityDisplay<Length, LengthUnit> Length::display_as(LengthUnit unit)
    {
        return QuantityDisplay<Length, LengthUnit>(*this, unit);
    }

    QuantityDisplay<Length, LengthUnit> Length::display_as_precision(LengthUnit unit, int precision)
    {
        return QuantityDisplay<Length, LengthUnit>(*this, unit, precision);
    }

    bool operator==(Length left, Length right)
    {
        return left._meters == right._meters;
    }

} // namespace aztro::physics::length
