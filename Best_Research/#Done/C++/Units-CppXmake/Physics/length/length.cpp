#include "Physics/length/length.hpp"

#include "Physics/detail/math.hpp"

namespace Physics::length
{

    Length::Length(double meters) : _meters(meters)
    {
    }

    Length Length::fromRawSi(double meters)
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

    double Length::rawSi()
    {
        return _meters;
    }

    double Length::asMeters()
    {
        return _meters;
    }

    double Length::asKilometers()
    {
        return _meters / 1'000.0;
    }

    double Length::asCentimeters()
    {
        return _meters * 100.0;
    }

    double Length::asMillimeters()
    {
        return _meters * 1'000.0;
    }

    double Length::asMicrometers()
    {
        return _meters * 1'000'000.0;
    }

    double Length::asNanometers()
    {
        return _meters * 1'000'000'000.0;
    }

    bool Length::approximatelyEquals(Length other, double epsilon)
    {
        return detail::absolute(_meters - other._meters) <= epsilon;
    }

    QuantityDisplay<Length, LengthUnit> Length::displayAs(LengthUnit unit)
    {
        return QuantityDisplay<Length, LengthUnit>(*this, unit);
    }

    QuantityDisplay<Length, LengthUnit> Length::displayAsPrecision(LengthUnit unit, int precision)
    {
        return QuantityDisplay<Length, LengthUnit>(*this, unit, precision);
    }

    bool operator==(Length left, Length right)
    {
        return left._meters == right._meters;
    }

} // namespace Physics::length
