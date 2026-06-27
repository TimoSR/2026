#pragma once

#include <optional>

#include "units/detail/quantity_display.hpp"
#include "units/forward.hpp"

namespace units {

enum class ForceUnit {
    Newtons,
    Millinewtons,
    Kilonewtons,
};

class Force {
public:
    static Force fromRawSi(double newtons);

    static Force newtons(double value);

    static Force millinewtons(double value);

    static Force kilonewtons(double value);

    [[nodiscard]] double rawSi() const;

    [[nodiscard]] double asNewtons() const;

    [[nodiscard]] double asMillinewtons() const;

    [[nodiscard]] double asKilonewtons() const;

    [[nodiscard]] bool approximatelyEquals(Force other, double epsilon) const;

    [[nodiscard]] QuantityDisplay<Force, ForceUnit> displayAs(ForceUnit unit) const;

    [[nodiscard]] QuantityDisplay<Force, ForceUnit> displayAsPrecision(ForceUnit unit, int precision) const;

    [[nodiscard]] std::optional<Acceleration> checkedDivMass(Mass mass) const;
    [[nodiscard]] std::optional<Mass> checkedDivAcceleration(Acceleration acceleration) const;

    friend bool operator==(Force left, Force right) = default;

private:
    explicit Force(double newtons);

    double newtons_;
};

} // namespace units
