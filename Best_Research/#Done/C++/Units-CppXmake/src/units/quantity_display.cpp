#include "units/detail/quantity_display.hpp"

#include "units/acceleration.hpp"
#include "units/force.hpp"
#include "units/length.hpp"
#include "units/mass.hpp"
#include "units/time.hpp"
#include "units/velocity.hpp"

namespace units
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

    template class QuantityDisplay<Length, LengthUnit>;
    template class QuantityDisplay<Time, TimeUnit>;
    template class QuantityDisplay<Mass, MassUnit>;
    template class QuantityDisplay<Velocity, VelocityUnit>;
    template class QuantityDisplay<Acceleration, AccelerationUnit>;
    template class QuantityDisplay<Force, ForceUnit>;

} // namespace units
