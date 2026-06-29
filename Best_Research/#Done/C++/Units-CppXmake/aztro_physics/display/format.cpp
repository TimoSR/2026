module;

#include <format>

export module aztro_physics:format;

import :display;
import :time;
import :mass;
import :acceleration;
import :velocity;
import :length;
import :force;

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

export template <typename Quantity, typename Unit> struct std::formatter<aztro::physics::QuantityDisplay<Quantity, Unit>, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, display);
        }
};

export template <> struct std::formatter<aztro::physics::length::Length, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::length::Length value, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::length::LengthUnit::Meters));
        }
};

export template <> struct std::formatter<aztro::physics::time::Time, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::time::Time value, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::time::TimeUnit::Seconds));
        }
};

export template <> struct std::formatter<aztro::physics::mass::Mass, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::mass::Mass value, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::mass::MassUnit::Kilograms));
        }
};

export template <> struct std::formatter<aztro::physics::velocity::Velocity, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::velocity::Velocity value, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::velocity::VelocityUnit::MetersPerSecond));
        }
};

export template <> struct std::formatter<aztro::physics::acceleration::Acceleration, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::acceleration::Acceleration value, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::acceleration::AccelerationUnit::MetersPerSecondSquared));
        }
};

export template <> struct std::formatter<aztro::physics::force::Force, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::force::Force value, std::format_context& context) const
        {
            return aztro::physics::detail::format_quantity_display(context, value.display_as(aztro::physics::force::ForceUnit::Newtons));
        }
};
