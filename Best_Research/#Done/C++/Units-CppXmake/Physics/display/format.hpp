#pragma once

#include <format>

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/detail/quantity_display.hpp"
#include "Physics/force/force.hpp"
#include "Physics/length/length.hpp"
#include "Physics/mass/mass.hpp"
#include "Physics/time/time.hpp"
#include "Physics/velocity/velocity.hpp"

template <typename Quantity, typename Unit> struct std::formatter<Physics::QuantityDisplay<Quantity, Unit>, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const;
};

template <> struct std::formatter<Physics::length::Length, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::length::Length value, std::format_context& context) const;
};

template <> struct std::formatter<Physics::time::Time, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::time::Time value, std::format_context& context) const;
};

template <> struct std::formatter<Physics::mass::Mass, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::mass::Mass value, std::format_context& context) const;
};

template <> struct std::formatter<Physics::velocity::Velocity, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::velocity::Velocity value, std::format_context& context) const;
};

template <> struct std::formatter<Physics::acceleration::Acceleration, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::acceleration::Acceleration value, std::format_context& context) const;
};

template <> struct std::formatter<Physics::force::Force, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(Physics::force::Force value, std::format_context& context) const;
};
