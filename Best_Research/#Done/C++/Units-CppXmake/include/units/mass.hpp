#pragma once

#include "units/detail/quantity_display.hpp"

namespace units {

enum class MassUnit {
    Kilograms,
    Grams,
    Milligrams,
    Micrograms,
    Tons,
};

class Mass {
public:
    static Mass fromRawSi(double kilograms);

    static Mass kilograms(double value);

    static Mass kilogram(double value);

    static Mass grams(double value);

    static Mass milligrams(double value);

    static Mass micrograms(double value);

    static Mass tons(double value);

    [[nodiscard]] double rawSi() const;

    [[nodiscard]] double asKilograms() const;

    [[nodiscard]] double asGrams() const;

    [[nodiscard]] double asMilligrams() const;

    [[nodiscard]] double asMicrograms() const;

    [[nodiscard]] double asTons() const;

    [[nodiscard]] bool approximatelyEquals(Mass other, double epsilon) const;

    [[nodiscard]] QuantityDisplay<Mass, MassUnit> displayAs(MassUnit unit) const;

    [[nodiscard]] QuantityDisplay<Mass, MassUnit> displayAsPrecision(MassUnit unit, int precision) const;

    friend bool operator==(Mass left, Mass right) = default;

private:
    explicit Mass(double kilograms);

    double kilograms_;
};

} // namespace units
