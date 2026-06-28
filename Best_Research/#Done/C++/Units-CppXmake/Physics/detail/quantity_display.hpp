#pragma once

#include <optional>

namespace physics
{

    template <typename Quantity, typename Unit> class QuantityDisplay
    {
        public:
            QuantityDisplay(Quantity value, Unit unit, std::optional<int> precision = std::nullopt);

            Quantity value();

            Unit unit();

            std::optional<int> precision();

        private:
            Quantity value_;
            Unit unit_;
            std::optional<int> precision_;
    };

} // namespace physics
