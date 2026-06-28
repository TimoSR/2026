#include "Physics/detail/quantity_display.hpp"

#include "Physics/acceleration/acceleration.hpp"
#include "Physics/force/force.hpp"
#include "Physics/length/length.hpp"
#include "Physics/mass/mass.hpp"
#include "Physics/time/time.hpp"
#include "Physics/velocity/velocity.hpp"

namespace Physics
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

    template class QuantityDisplay<length::Length, length::LengthUnit>;
    template class QuantityDisplay<time::Time, time::TimeUnit>;
    template class QuantityDisplay<mass::Mass, mass::MassUnit>;
    template class QuantityDisplay<velocity::Velocity, velocity::VelocityUnit>;
    template class QuantityDisplay<acceleration::Acceleration, acceleration::AccelerationUnit>;
    template class QuantityDisplay<force::Force, force::ForceUnit>;

} // namespace Physics
