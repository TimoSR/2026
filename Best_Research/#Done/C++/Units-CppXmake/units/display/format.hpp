#pragma once

#include <format>

#include "units/acceleration/acceleration.hpp"
#include "units/detail/quantity_display.hpp"
#include "units/force/force.hpp"
#include "units/length/length.hpp"
#include "units/mass/mass.hpp"
#include "units/time/time.hpp"
#include "units/velocity/velocity.hpp"

template <typename Quantity, typename Unit> struct std::formatter<units::QuantityDisplay<Quantity, Unit>, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const;
};

template <> struct std::formatter<units::Length, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::Length value, std::format_context& context) const;
};

template <> struct std::formatter<units::Time, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::Time value, std::format_context& context) const;
};

template <> struct std::formatter<units::Mass, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::Mass value, std::format_context& context) const;
};

template <> struct std::formatter<units::Velocity, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::Velocity value, std::format_context& context) const;
};

template <> struct std::formatter<units::Acceleration, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::Acceleration value, std::format_context& context) const;
};

template <> struct std::formatter<units::Force, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::Force value, std::format_context& context) const;
};
