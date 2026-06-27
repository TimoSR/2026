#pragma once

#include <optional>

#include "units/detail/quantity_display.hpp"
#include "units/forward.hpp"

namespace units {

enum class VelocityUnit {
    MetersPerSecond,
    KilometersPerHour,
};

class Velocity {
public:
    static Velocity fromRawSi(double metersPerSecond);

    static Velocity metersPerSecond(double value);

    static Velocity kilometersPerHour(double value);

    [[nodiscard]] double rawSi() const;

    [[nodiscard]] double asMetersPerSecond() const;

    [[nodiscard]] double asKilometersPerHour() const;

    [[nodiscard]] bool approximatelyEquals(Velocity other, double epsilon) const;

    [[nodiscard]] QuantityDisplay<Velocity, VelocityUnit> displayAs(VelocityUnit unit) const;

    [[nodiscard]] QuantityDisplay<Velocity, VelocityUnit> displayAsPrecision(
        VelocityUnit unit,
        int precision
    ) const;

    [[nodiscard]] std::optional<Acceleration> checkedDivTime(Time time) const;
    [[nodiscard]] std::optional<Time> checkedDivAcceleration(Acceleration acceleration) const;

    friend bool operator==(Velocity left, Velocity right) = default;

private:
    explicit Velocity(double metersPerSecond);

    double metersPerSecond_;
};

} // namespace units
