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

template <> struct std::formatter<units::length::Length, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::length::Length value, std::format_context& context) const;
};

template <> struct std::formatter<units::time::Time, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::time::Time value, std::format_context& context) const;
};

template <> struct std::formatter<units::mass::Mass, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::mass::Mass value, std::format_context& context) const;
};

template <> struct std::formatter<units::velocity::Velocity, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::velocity::Velocity value, std::format_context& context) const;
};

template <> struct std::formatter<units::acceleration::Acceleration, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::acceleration::Acceleration value, std::format_context& context) const;
};

template <> struct std::formatter<units::force::Force, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(units::force::Force value, std::format_context& context) const;
};
