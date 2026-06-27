#pragma once

#include "units/detail/quantity_display.hpp"

namespace units {

enum class AccelerationUnit {
    MetersPerSecondSquared,
    StandardGravity,
};

class Acceleration {
public:
    static constexpr double StandardGravityMetersPerSecondSquared = 9.80665;

    static Acceleration fromRawSi(double metersPerSecondSquared);

    static Acceleration metersPerSecondSquared(double value);

    static Acceleration standardGravity(double value);

    double rawSi() const;

    double asMetersPerSecondSquared() const;

    [[nodiscard]] double asStandardGravity() const;

    [[nodiscard]] bool approximatelyEquals(Acceleration other, double epsilon) const;

    [[nodiscard]] QuantityDisplay<Acceleration, AccelerationUnit> displayAs(AccelerationUnit unit) const;

    [[nodiscard]] QuantityDisplay<Acceleration, AccelerationUnit> displayAsPrecision(
        AccelerationUnit unit,
        int precision
    ) const;

    friend bool operator==(Acceleration left, Acceleration right) = default;

private:
    explicit Acceleration(double metersPerSecondSquared);

    double metersPerSecondSquared_;
};

} // namespace units
