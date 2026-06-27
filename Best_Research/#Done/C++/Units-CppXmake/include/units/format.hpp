#pragma once

#include <format>

#include "units/display.hpp"

namespace units::detail {

template <typename ParseContext>
constexpr auto parse_empty_format_spec(ParseContext& context) {
    auto iterator = context.begin();

    if (iterator != context.end() && *iterator != '}') {
        throw std::format_error("units formatters do not accept custom format specs");
    }

    return iterator;
}

template <typename FormatContext, typename Quantity, typename Unit>
auto format_quantity_display(
    FormatContext& context,
    QuantityDisplay<Quantity, Unit> display
) {
    const auto value = valueInUnit(display.value(), display.unit());
    const auto unitSymbol = symbol(display.unit());

    if (display.precision().has_value()) {
        return std::format_to(context.out(), "{:.{}f} {}", value, *display.precision(), unitSymbol);
    }

    return std::format_to(context.out(), "{} {}", value, unitSymbol);
}

} // namespace units::detail

template <typename Quantity, typename Unit>
struct std::formatter<units::QuantityDisplay<Quantity, Unit>, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::QuantityDisplay<Quantity, Unit> display, FormatContext& context) const {
        return units::detail::format_quantity_display(context, display);
    }
};

template <>
struct std::formatter<units::Length, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::Length value, FormatContext& context) const {
        return units::detail::format_quantity_display(
            context,
            value.displayAs(units::LengthUnit::Meters)
        );
    }
};

template <>
struct std::formatter<units::Time, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::Time value, FormatContext& context) const {
        return units::detail::format_quantity_display(
            context,
            value.displayAs(units::TimeUnit::Seconds)
        );
    }
};

template <>
struct std::formatter<units::Mass, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::Mass value, FormatContext& context) const {
        return units::detail::format_quantity_display(
            context,
            value.displayAs(units::MassUnit::Kilograms)
        );
    }
};

template <>
struct std::formatter<units::Velocity, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::Velocity value, FormatContext& context) const {
        return units::detail::format_quantity_display(
            context,
            value.displayAs(units::VelocityUnit::MetersPerSecond)
        );
    }
};

template <>
struct std::formatter<units::Acceleration, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::Acceleration value, FormatContext& context) const {
        return units::detail::format_quantity_display(
            context,
            value.displayAs(units::AccelerationUnit::MetersPerSecondSquared)
        );
    }
};

template <>
struct std::formatter<units::Force, char> {
    template <typename ParseContext>
    constexpr auto parse(ParseContext& context) {
        return units::detail::parse_empty_format_spec(context);
    }

    template <typename FormatContext>
    auto format(units::Force value, FormatContext& context) const {
        return units::detail::format_quantity_display(
            context,
            value.displayAs(units::ForceUnit::Newtons)
        );
    }
};
