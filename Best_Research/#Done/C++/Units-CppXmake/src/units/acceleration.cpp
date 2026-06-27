#include "units/acceleration.hpp"

#include "units/detail/math.hpp"

namespace units {

Acceleration::Acceleration(double metersPerSecondSquared)
    : metersPerSecondSquared_(metersPerSecondSquared) {
}

Acceleration Acceleration::fromRawSi(double metersPerSecondSquared) {
    return Acceleration(metersPerSecondSquared);
}

Acceleration Acceleration::metersPerSecondSquared(double value) {
    return Acceleration(value);
}

Acceleration Acceleration::standardGravity(double value) {
    return Acceleration(value * StandardGravityMetersPerSecondSquared);
}

double Acceleration::rawSi() const {
    return metersPerSecondSquared_;
}

double Acceleration::asMetersPerSecondSquared() const {
    return metersPerSecondSquared_;
}

double Acceleration::asStandardGravity() const {
    return metersPerSecondSquared_ / StandardGravityMetersPerSecondSquared;
}

bool Acceleration::approximatelyEquals(Acceleration other, double epsilon) const {
    return detail::absolute(metersPerSecondSquared_ - other.metersPerSecondSquared_) <= epsilon;
}

QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::displayAs(
    AccelerationUnit unit
) const {
    return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit);
}

QuantityDisplay<Acceleration, AccelerationUnit> Acceleration::displayAsPrecision(
    AccelerationUnit unit,
    int precision
) const {
    return QuantityDisplay<Acceleration, AccelerationUnit>(*this, unit, precision);
}

} // namespace units
