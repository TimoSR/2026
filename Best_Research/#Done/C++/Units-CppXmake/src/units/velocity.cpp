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

double Velocity::rawSi() {
    return metersPerSecond_;
}

double Velocity::asMetersPerSecond() {
    return metersPerSecond_;
}

double Velocity::asKilometersPerHour() {
    return metersPerSecond_ * 3.6;
}

bool Velocity::approximatelyEquals(Velocity other, double epsilon) {
    return detail::absolute(metersPerSecond_ - other.metersPerSecond_) <= epsilon;
}

QuantityDisplay<Velocity, VelocityUnit> Velocity::displayAs(VelocityUnit unit) {
    return QuantityDisplay<Velocity, VelocityUnit>(*this, unit);
}

QuantityDisplay<Velocity, VelocityUnit> Velocity::displayAsPrecision(
    VelocityUnit unit,
    int precision
) {
    return QuantityDisplay<Velocity, VelocityUnit>(*this, unit, precision);
}

bool operator==(Velocity left, Velocity right) {
    return left.metersPerSecond_ == right.metersPerSecond_;
}

} // namespace units
