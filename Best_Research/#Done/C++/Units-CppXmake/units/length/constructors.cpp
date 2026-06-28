#include "units/constructors/constructors.hpp"

namespace units::length
{

    Length meters(double value)
    {
        return Length::meters(value);
    }

    Length kilometers(double value)
    {
        return Length::kilometers(value);
    }

    Length centimeters(double value)
    {
        return Length::centimeters(value);
    }

    Length millimeters(double value)
    {
        return Length::millimeters(value);
    }

    Length micrometers(double value)
    {
        return Length::micrometers(value);
    }

    Length nanometers(double value)
    {
        return Length::nanometers(value);
    }

} // namespace units::length
