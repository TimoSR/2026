module;
#include <optional>

export module aztro_physics:quantity_display;

export {
    namespace aztro::physics
    {

        template <typename Quantity, typename Unit> class QuantityDisplay
        {
            private:
                std::optional<Quantity> value_;
                Unit unit_;
                std::optional<int> precision_;

            public:
                QuantityDisplay(Quantity value, Unit unit, std::optional<int> precision = std::nullopt)
                {
                    value_ = value;
                    unit_ = unit;
                    precision_ = precision;
                }

            public:
                Quantity value();

                Unit unit();

                std::optional<int> precision();
        };

    } // namespace aztro::physics

    namespace aztro::physics
    {

        template <typename Quantity, typename Unit> Quantity QuantityDisplay<Quantity, Unit>::value()
        {
            return *value_;
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
