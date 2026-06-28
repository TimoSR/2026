#include "units/display/format.hpp"

#include "units/acceleration/acceleration.hpp"
#include "units/display/display.hpp"
#include "units/force/force.hpp"
#include "units/length/length.hpp"
#include "units/mass/mass.hpp"
#include "units/time/time.hpp"
#include "units/velocity/velocity.hpp"

namespace units::detail
{

    template <typename Quantity, typename Unit> std::format_context::iterator formatQuantityDisplay(std::format_context& context, QuantityDisplay<Quantity, Unit> display)
    {
        auto value = valueInUnit(display.value(), display.unit());
        auto unitSymbol = symbol(display.unit());

        if (display.precision().has_value())
        {
            return std::format_to(context.out(), "{:.{}f} {}", value, *display.precision(), unitSymbol);
        }

        return std::format_to(context.out(), "{} {}", value, unitSymbol);
    }

} // namespace units::detail

template <typename Quantity, typename Unit>
std::format_context::iterator std::formatter<units::QuantityDisplay<Quantity, Unit>, char>::format(units::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, display);
}

std::format_context::iterator std::formatter<units::length::Length, char>::format(units::length::Length value, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::length::LengthUnit::Meters));
}

std::format_context::iterator std::formatter<units::time::Time, char>::format(units::time::Time value, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::time::TimeUnit::Seconds));
}

std::format_context::iterator std::formatter<units::mass::Mass, char>::format(units::mass::Mass value, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::mass::MassUnit::Kilograms));
}

std::format_context::iterator std::formatter<units::velocity::Velocity, char>::format(units::velocity::Velocity value, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::velocity::VelocityUnit::MetersPerSecond));
}

std::format_context::iterator std::formatter<units::acceleration::Acceleration, char>::format(units::acceleration::Acceleration value, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::acceleration::AccelerationUnit::MetersPerSecondSquared));
}

std::format_context::iterator std::formatter<units::force::Force, char>::format(units::force::Force value, std::format_context& context) const
{
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::force::ForceUnit::Newtons));
}

template struct std::formatter<units::QuantityDisplay<units::length::Length, units::length::LengthUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::time::Time, units::time::TimeUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::mass::Mass, units::mass::MassUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::velocity::Velocity, units::velocity::VelocityUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::acceleration::Acceleration, units::acceleration::AccelerationUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::force::Force, units::force::ForceUnit>, char>;
