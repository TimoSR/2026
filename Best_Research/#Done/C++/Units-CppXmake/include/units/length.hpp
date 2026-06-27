#pragma once

#include <optional>

#include "units/detail/quantity_display.hpp"
#include "units/forward.hpp"

namespace units {

enum class LengthUnit {
    Meters,
    Kilometers,
    Centimeters,
    Millimeters,
    Micrometers,
    Nanometers,
};

class Length {
public:
    static Length fromRawSi(double meters);

    static Length meters(double value);

    static Length kilometers(double value);

    static Length centimeters(double value);

    static Length millimeters(double value);

    static Length micrometers(double value);

    static Length nanometers(double value);

    [[nodiscard]] double rawSi() const;

    [[nodiscard]] double asMeters() const;

    [[nodiscard]] double asKilometers() const;

    [[nodiscard]] double asCentimeters() const;

    [[nodiscard]] double asMillimeters() const;

    [[nodiscard]] double asMicrometers() const;

    [[nodiscard]] double asNanometers() const;

    [[nodiscard]] bool approximatelyEquals(Length other, double epsilon) const;

    [[nodiscard]] QuantityDisplay<Length, LengthUnit> displayAs(LengthUnit unit) const;

    [[nodiscard]] QuantityDisplay<Length, LengthUnit> displayAsPrecision(LengthUnit unit, int precision) const;

    [[nodiscard]] std::optional<Velocity> checkedDivTime(Time time) const;
    [[nodiscard]] std::optional<Time> checkedDivVelocity(Velocity velocity) const;

    friend bool operator==(Length left, Length right) = default;

private:
    explicit Length(double meters);

    double meters_;
};

} // namespace units
