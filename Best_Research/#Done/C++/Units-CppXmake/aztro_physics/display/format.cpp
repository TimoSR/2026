module;

#include <format>
#include <iomanip>
#include <limits>
#include <optional>
#include <ostream>

module aztro_physics;

namespace aztro::physics::detail
{

    template <typename Quantity, typename Unit> std::format_context::iterator format_quantity_display(std::format_context& context, QuantityDisplay<Quantity, Unit> display)
    {
        auto value = value_in_unit(display.value(), display.unit());
        auto unit_symbol = symbol(display.unit());

        if (display.precision().has_value())
        {
            return std::format_to(context.out(), "{:.{}f} {}", value, *display.precision(), unit_symbol);
        }

        return std::format_to(context.out(), "{} {}", value, unit_symbol);
    }

} // namespace aztro::physics::detail

template <typename Quantity, typename Unit>
std::format_context::iterator std::formatter<aztro::physics::QuantityDisplay<Quantity, Unit>, char>::format(aztro::physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, display);
}

std::format_context::iterator std::formatter<aztro::physics::length::Length, char>::format(aztro::physics::length::Length value, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::length::LengthUnit::Meters));
}

std::format_context::iterator std::formatter<aztro::physics::time::Time, char>::format(aztro::physics::time::Time value, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::time::TimeUnit::Seconds));
}

std::format_context::iterator std::formatter<aztro::physics::mass::Mass, char>::format(aztro::physics::mass::Mass value, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::mass::MassUnit::Kilograms));
}

std::format_context::iterator std::formatter<aztro::physics::velocity::Velocity, char>::format(aztro::physics::velocity::Velocity value, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::velocity::VelocityUnit::MetersPerSecond));
}

std::format_context::iterator std::formatter<aztro::physics::acceleration::Acceleration, char>::format(aztro::physics::acceleration::Acceleration value, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::acceleration::AccelerationUnit::MetersPerSecondSquared));
}

std::format_context::iterator std::formatter<aztro::physics::force::Force, char>::format(aztro::physics::force::Force value, std::format_context& context) const
{
    return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::force::ForceUnit::Newtons));
}

template struct std::formatter<aztro::physics::QuantityDisplay<aztro::physics::length::Length, aztro::physics::length::LengthUnit>, char>;
template struct std::formatter<aztro::physics::QuantityDisplay<aztro::physics::time::Time, aztro::physics::time::TimeUnit>, char>;
template struct std::formatter<aztro::physics::QuantityDisplay<aztro::physics::mass::Mass, aztro::physics::mass::MassUnit>, char>;
template struct std::formatter<aztro::physics::QuantityDisplay<aztro::physics::velocity::Velocity, aztro::physics::velocity::VelocityUnit>, char>;
template struct std::formatter<aztro::physics::QuantityDisplay<aztro::physics::acceleration::Acceleration, aztro::physics::acceleration::AccelerationUnit>, char>;
template struct std::formatter<aztro::physics::QuantityDisplay<aztro::physics::force::Force, aztro::physics::force::ForceUnit>, char>;
