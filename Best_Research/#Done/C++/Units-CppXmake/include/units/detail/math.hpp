#pragma once

namespace units::detail {

[[nodiscard]] constexpr double absolute(double value) {
    return value < 0.0 ? -value : value;
}

} // namespace units::detail
