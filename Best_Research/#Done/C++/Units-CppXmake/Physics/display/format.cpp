#include "Physics/display/format.hpp"

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/display/display.hpp"
#include "Physics/force/force.hpp"
#include "Physics/length/length.hpp"
#include "Physics/mass/mass.hpp"
#include "Physics/time/time.hpp"
#include "Physics/velocity/velocity.hpp"

namespace Physics::detail
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

} // namespace Physics::detail

template <typename Quantity, typename Unit>
std::format_context::iterator std::formatter<Physics::QuantityDisplay<Quantity, Unit>, char>::format(Physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, display);
}

std::format_context::iterator std::formatter<Physics::length::Length, char>::format(Physics::length::Length value, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, value.display_as(Physics::length::LengthUnit::Meters));
}

std::format_context::iterator std::formatter<Physics::time::Time, char>::format(Physics::time::Time value, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, value.display_as(Physics::time::TimeUnit::Seconds));
}

std::format_context::iterator std::formatter<Physics::mass::Mass, char>::format(Physics::mass::Mass value, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, value.display_as(Physics::mass::MassUnit::Kilograms));
}

std::format_context::iterator std::formatter<Physics::velocity::Velocity, char>::format(Physics::velocity::Velocity value, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, value.display_as(Physics::velocity::VelocityUnit::MetersPerSecond));
}

std::format_context::iterator std::formatter<Physics::acceleration::Acceleration, char>::format(Physics::acceleration::Acceleration value, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, value.display_as(Physics::acceleration::AccelerationUnit::MetersPerSecondSquared));
}

std::format_context::iterator std::formatter<Physics::force::Force, char>::format(Physics::force::Force value, std::format_context& context) const
{
    return Physics::detail::format_quantity_display(context, value.display_as(Physics::force::ForceUnit::Newtons));
}

template struct std::formatter<Physics::QuantityDisplay<Physics::length::Length, Physics::length::LengthUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::time::Time, Physics::time::TimeUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::mass::Mass, Physics::mass::MassUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::velocity::Velocity, Physics::velocity::VelocityUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::acceleration::Acceleration, Physics::acceleration::AccelerationUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::force::Force, Physics::force::ForceUnit>, char>;
