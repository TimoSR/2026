#include "units/length.hpp"

#include "units/detail/math.hpp"

namespace units
{

    Length::Length(double meters) : meters_(meters)
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
        return meters_;
    }

    double Length::asMeters()
    {
        return meters_;
    }

    double Length::asKilometers()
    {
        return meters_ / 1'000.0;
    }

    double Length::asCentimeters()
    {
        return meters_ * 100.0;
    }

    double Length::asMillimeters()
    {
        return meters_ * 1'000.0;
    }

    double Length::asMicrometers()
    {
        return meters_ * 1'000'000.0;
    }

    double Length::asNanometers()
    {
        return meters_ * 1'000'000'000.0;
    }

    bool Length::approximatelyEquals(Length other, double epsilon)
    {
        return detail::absolute(meters_ - other.meters_) <= epsilon;
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
        return left.meters_ == right.meters_;
    }

} // namespace units
