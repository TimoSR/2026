module;
#include <optional>

export module aztro_physics:quantity_display;

export {
    namespace aztro::physics
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

    } // namespace aztro::physics

    namespace aztro::physics
    {

        template <typename Quantity, typename Unit>
        QuantityDisplay<Quantity, Unit>::QuantityDisplay(Quantity value, Unit unit, std::optional<int> precision) : value_(value), unit_(unit), precision_(precision)
        {
        }

        template <typename Quantity, typename Unit> Quantity QuantityDisplay<Quantity, Unit>::value()
        {
            return value_;
        }

        template <typename Quantity, typename Unit> Unit QuantityDisplay<Quantity, Unit>::unit()
        {
            return unit_;
        }

        template <typename Quantity, typename Unit> std::optional<int> QuantityDisplay<Quantity, Unit>::precision()
        {
            return precision_;
        }

    } // namespace aztro::physics
}
