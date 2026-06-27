#pragma once

#define UNITS_IMPLEMENT_SAME_QUANTITY_OPERATORS(Quantity)                                      \
    [[nodiscard]] inline constexpr Quantity operator+(Quantity left, Quantity right) {          \
        return Quantity::fromRawSi(left.rawSi() + right.rawSi());                               \
    }                                                                                           \
    [[nodiscard]] inline constexpr Quantity operator-(Quantity left, Quantity right) {          \
        return Quantity::fromRawSi(left.rawSi() - right.rawSi());                               \
    }                                                                                           \
    [[nodiscard]] inline constexpr Quantity operator-(Quantity value) {                         \
        return Quantity::fromRawSi(-value.rawSi());                                             \
    }                                                                                           \
    [[nodiscard]] inline constexpr Quantity operator*(Quantity value, double scalar) {          \
        return Quantity::fromRawSi(value.rawSi() * scalar);                                     \
    }                                                                                           \
    [[nodiscard]] inline constexpr Quantity operator*(double scalar, Quantity value) {          \
        return Quantity::fromRawSi(scalar * value.rawSi());                                     \
    }                                                                                           \
    [[nodiscard]] inline constexpr Quantity operator/(Quantity value, double scalar) {          \
        return Quantity::fromRawSi(value.rawSi() / scalar);                                     \
    }                                                                                           \
    [[nodiscard]] inline constexpr double operator/(Quantity left, Quantity right) {            \
        return left.rawSi() / right.rawSi();                                                    \
    }
