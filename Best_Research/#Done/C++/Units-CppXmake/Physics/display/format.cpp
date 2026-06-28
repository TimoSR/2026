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

} // namespace Physics::detail

template <typename Quantity, typename Unit>
std::format_context::iterator std::formatter<Physics::QuantityDisplay<Quantity, Unit>, char>::format(Physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, display);
}

std::format_context::iterator std::formatter<Physics::length::Length, char>::format(Physics::length::Length value, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, value.displayAs(Physics::length::LengthUnit::Meters));
}

std::format_context::iterator std::formatter<Physics::time::Time, char>::format(Physics::time::Time value, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, value.displayAs(Physics::time::TimeUnit::Seconds));
}

std::format_context::iterator std::formatter<Physics::mass::Mass, char>::format(Physics::mass::Mass value, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, value.displayAs(Physics::mass::MassUnit::Kilograms));
}

std::format_context::iterator std::formatter<Physics::velocity::Velocity, char>::format(Physics::velocity::Velocity value, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, value.displayAs(Physics::velocity::VelocityUnit::MetersPerSecond));
}

std::format_context::iterator std::formatter<Physics::acceleration::Acceleration, char>::format(Physics::acceleration::Acceleration value, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, value.displayAs(Physics::acceleration::AccelerationUnit::MetersPerSecondSquared));
}

std::format_context::iterator std::formatter<Physics::force::Force, char>::format(Physics::force::Force value, std::format_context& context) const
{
    return Physics::detail::formatQuantityDisplay(context, value.displayAs(Physics::force::ForceUnit::Newtons));
}

template struct std::formatter<Physics::QuantityDisplay<Physics::length::Length, Physics::length::LengthUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::time::Time, Physics::time::TimeUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::mass::Mass, Physics::mass::MassUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::velocity::Velocity, Physics::velocity::VelocityUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::acceleration::Acceleration, Physics::acceleration::AccelerationUnit>, char>;
template struct std::formatter<Physics::QuantityDisplay<Physics::force::Force, Physics::force::ForceUnit>, char>;
