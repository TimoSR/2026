#pragma once

#include <optional>

namespace physics
{

    template <typename Quantity, typename Unit> class QuantityDisplay
    {
        private:
            std::optional<Quantity> value_;
            Unit unit_;
            std::optional<int> precision_;

        public:
            QuantityDisplay(Quantity value, Unit unit, std::optional<int> precision = std::nullopt);

        public:
            Quantity value();

            Unit unit();

            std::optional<int> precision();
    };

} // namespace physics
