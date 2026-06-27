#pragma once

#include <optional>

namespace units {

template <typename Quantity, typename Unit>
class QuantityDisplay {
public:
    constexpr QuantityDisplay(Quantity value, Unit unit, std::optional<int> precision = std::nullopt)
        : value_(value), unit_(unit), precision_(precision) {
    }

    [[nodiscard]] constexpr Quantity value() const {
        return value_;
    }

    [[nodiscard]] constexpr Unit unit() const {
        return unit_;
    }

    [[nodiscard]] constexpr std::optional<int> precision() const {
        return precision_;
    }

private:
    Quantity value_;
    Unit unit_;
    std::optional<int> precision_;
};

} // namespace units
