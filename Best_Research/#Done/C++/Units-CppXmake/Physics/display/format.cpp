#include "physics/display/format.hpp"

#include "physics/acceleration/acceleration.hpp"
#include "physics/display/display.hpp"
#include "physics/force/force.hpp"
#include "physics/length/length.hpp"
#include "physics/mass/mass.hpp"
#include "physics/time/time.hpp"
#include "physics/velocity/velocity.hpp"

namespace physics::detail
{

    template <typename Quantity, typename Unit> 
    std::format_context::iterator format_quantity_display(std::format_context& context, QuantityDisplay<Quantity, Unit> display)
    {
        auto value = value_in_unit(display.value(), display.unit());
        auto unit_symbol = symbol(display.unit());

        if (display.precision().has_value())
        {
            return std::format_to(context.out(), "{:.{}f} {}", value, *display.precision(), unit_symbol);
        }

        return std::format_to(context.out(), "{} {}", value, unit_symbol);
    }

} // namespace physics::detail

template <typename Quantity, typename Unit>
std::format_context::iterator std::formatter<physics::QuantityDisplay<Quantity, Unit>, char>::format(physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, display);
}

std::format_context::iterator std::formatter<physics::length::Length, char>::format(physics::length::Length value, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, value.display_as(physics::length::LengthUnit::Meters));
}

std::format_context::iterator std::formatter<physics::time::Time, char>::format(physics::time::Time value, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, value.display_as(physics::time::TimeUnit::Seconds));
}

std::format_context::iterator std::formatter<physics::mass::Mass, char>::format(physics::mass::Mass value, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, value.display_as(physics::mass::MassUnit::Kilograms));
}

std::format_context::iterator std::formatter<physics::velocity::Velocity, char>::format(physics::velocity::Velocity value, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, value.display_as(physics::velocity::VelocityUnit::MetersPerSecond));
}

std::format_context::iterator std::formatter<physics::acceleration::Acceleration, char>::format(physics::acceleration::Acceleration value, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, value.display_as(physics::acceleration::AccelerationUnit::MetersPerSecondSquared));
}

std::format_context::iterator std::formatter<physics::force::Force, char>::format(physics::force::Force value, std::format_context& context) const
{
    return physics::detail::format_quantity_display(context, value.display_as(physics::force::ForceUnit::Newtons));
}

template struct std::formatter<physics::QuantityDisplay<physics::length::Length, physics::length::LengthUnit>, char>;
template struct std::formatter<physics::QuantityDisplay<physics::time::Time, physics::time::TimeUnit>, char>;
template struct std::formatter<physics::QuantityDisplay<physics::mass::Mass, physics::mass::MassUnit>, char>;
template struct std::formatter<physics::QuantityDisplay<physics::velocity::Velocity, physics::velocity::VelocityUnit>, char>;
template struct std::formatter<physics::QuantityDisplay<physics::acceleration::Acceleration, physics::acceleration::AccelerationUnit>, char>;
template struct std::formatter<physics::QuantityDisplay<physics::force::Force, physics::force::ForceUnit>, char>;
