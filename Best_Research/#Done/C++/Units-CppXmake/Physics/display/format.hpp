#pragma once

#include <format>

#include "physics/acceleration/acceleration.hpp"
#include "physics/detail/quantity_display.hpp"
#include "physics/force/force.hpp"
#include "physics/length/length.hpp"
#include "physics/mass/mass.hpp"
#include "physics/time/time.hpp"
#include "physics/velocity/velocity.hpp"

template <typename Quantity, typename Unit> struct std::formatter<physics::QuantityDisplay<Quantity, Unit>, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const;
};

template <> struct std::formatter<physics::length::Length, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::length::Length value, std::format_context& context) const;
};

template <> struct std::formatter<physics::time::Time, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::time::Time value, std::format_context& context) const;
};

template <> struct std::formatter<physics::mass::Mass, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::mass::Mass value, std::format_context& context) const;
};

template <> struct std::formatter<physics::velocity::Velocity, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::velocity::Velocity value, std::format_context& context) const;
};

template <> struct std::formatter<physics::acceleration::Acceleration, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::acceleration::Acceleration value, std::format_context& context) const;
};

template <> struct std::formatter<physics::force::Force, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(physics::force::Force value, std::format_context& context) const;
};
