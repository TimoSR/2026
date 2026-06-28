module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro.physics;

namespace aztro::physics
{

    namespace detail
    {

        template <typename Quantity, typename Unit> std::ostream& write_display(std::ostream& stream, QuantityDisplay<Quantity, Unit> display)
        {
            auto flags = stream.flags();
            auto precision = stream.precision();

            if (display.precision().has_value())
            {
                stream << std::fixed << std::setprecision(*display.precision());
            }

            stream << value_in_unit(display.value(), display.unit()) << ' ' << symbol(display.unit());

            stream.flags(flags);
            stream.precision(precision);
            return stream;
        }

    } // namespace detail

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<length::Length, length::LengthUnit> display)
    {
        return detail::write_display(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<time::Time, time::TimeUnit> display)
    {
        return detail::write_display(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<mass::Mass, mass::MassUnit> display)
    {
        return detail::write_display(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<velocity::Velocity, velocity::VelocityUnit> display)
    {
        return detail::write_display(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit> display)
    {
        return detail::write_display(stream, display);
    }

    std::ostream& operator<<(std::ostream& stream, QuantityDisplay<force::Force, force::ForceUnit> display)
    {
        return detail::write_display(stream, display);
    }

} // namespace aztro::physics
