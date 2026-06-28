#include "units/format.hpp"

#include "units/acceleration.hpp"
#include "units/display.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units::detail {

template <typename Quantity, typename Unit>
std::format_context::iterator formatQuantityDisplay(
    std::format_context& context,
    QuantityDisplay<Quantity, Unit> display
) {
    auto value = valueInUnit(display.value(), display.unit());
    auto unitSymbol = symbol(display.unit());

    if (display.precision().has_value()) {
        return std::format_to(context.out(), "{:.{}f} {}", value, *display.precision(), unitSymbol);
    }

    return std::format_to(context.out(), "{} {}", value, unitSymbol);
}

} // namespace units::detail

template <typename Quantity, typename Unit>
std::format_context::iterator std::formatter<
    units::QuantityDisplay<Quantity, Unit>,
    char
>::format(units::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const {
    return units::detail::formatQuantityDisplay(context, display);
}

std::format_context::iterator std::formatter<units::Length, char>::format(
    units::Length value,
    std::format_context& context
) const {
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::LengthUnit::Meters));
}

std::format_context::iterator std::formatter<units::Time, char>::format(
    units::Time value,
    std::format_context& context
) const {
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::TimeUnit::Seconds));
}

std::format_context::iterator std::formatter<units::Mass, char>::format(
    units::Mass value,
    std::format_context& context
) const {
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::MassUnit::Kilograms));
}

std::format_context::iterator std::formatter<units::Velocity, char>::format(
    units::Velocity value,
    std::format_context& context
) const {
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::VelocityUnit::MetersPerSecond));
}

std::format_context::iterator std::formatter<units::Acceleration, char>::format(
    units::Acceleration value,
    std::format_context& context
) const {
    return units::detail::formatQuantityDisplay(
        context,
        value.displayAs(units::AccelerationUnit::MetersPerSecondSquared)
    );
}

std::format_context::iterator std::formatter<units::Force, char>::format(
    units::Force value,
    std::format_context& context
) const {
    return units::detail::formatQuantityDisplay(context, value.displayAs(units::ForceUnit::Newtons));
}

template struct std::formatter<units::QuantityDisplay<units::Length, units::LengthUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::Time, units::TimeUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::Mass, units::MassUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::Velocity, units::VelocityUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::Acceleration, units::AccelerationUnit>, char>;
template struct std::formatter<units::QuantityDisplay<units::Force, units::ForceUnit>, char>;
