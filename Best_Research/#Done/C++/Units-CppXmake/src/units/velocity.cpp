#include "units/velocity.hpp"

#include "units/detail/math.hpp"

namespace units {

Velocity::Velocity(double metersPerSecond) : metersPerSecond_(metersPerSecond) {
}

Velocity Velocity::fromRawSi(double metersPerSecond) {
    return Velocity(metersPerSecond);
}

Velocity Velocity::metersPerSecond(double value) {
    return Velocity(value);
}

Velocity Velocity::kilometersPerHour(double value) {
    return Velocity(value / 3.6);
}

double Velocity::rawSi() const {
    return metersPerSecond_;
}

double Velocity::asMetersPerSecond() const {
    return metersPerSecond_;
}

double Velocity::asKilometersPerHour() const {
    return metersPerSecond_ * 3.6;
}

bool Velocity::approximatelyEquals(Velocity other, double epsilon) const {
    return detail::absolute(metersPerSecond_ - other.metersPerSecond_) <= epsilon;
}

QuantityDisplay<Velocity, VelocityUnit> Velocity::displayAs(VelocityUnit unit) const {
    return QuantityDisplay<Velocity, VelocityUnit>(*this, unit);
}

QuantityDisplay<Velocity, VelocityUnit> Velocity::displayAsPrecision(
    VelocityUnit unit,
    int precision
) const {
    return QuantityDisplay<Velocity, VelocityUnit>(*this, unit, precision);
}

} // namespace units
