#include "Physics/display/display.hpp"

#include <iomanip>
#include <ostream>

namespace Physics
{

    namespace detail
    {

        template <typename Quantity, typename Unit> std::ostream& writeDisplay(std::ostream& stream, QuantityDisplay<Quantity, Unit> display)
        {
            auto flags = stream.flags();
            auto precision = stream.precision();

            if (display.precision().has_value())
            {
                stream << std::fixed << std::setprecision(*display.precision());
            }

            stream << valueInUnit(display.value(), display.unit()) << ' ' << symbol(display.unit());

            stream.flags(flags);
            stream.precision(precision);
            return stream;
        }

    } // namespace detail

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display)
    {
        return detail::writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display)
    {
        return detail::writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display)
    {
        return detail::writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display)
    {
        return detail::writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display)
    {
        return detail::writeDisplay(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display)
    {
        return detail::writeDisplay(stream, display);
    }

} // namespace Physics
