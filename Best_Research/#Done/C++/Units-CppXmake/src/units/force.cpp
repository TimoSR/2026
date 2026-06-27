#include "units/force.hpp"

#include "units/detail/math.hpp"

namespace units {

Force::Force(double newtons) : newtons_(newtons) {
}

Force Force::fromRawSi(double newtons) {
    return Force(newtons);
}

Force Force::newtons(double value) {
    return Force(value);
}

Force Force::millinewtons(double value) {
    return Force(value / 1'000.0);
}

Force Force::kilonewtons(double value) {
    return Force(value * 1'000.0);
}

double Force::rawSi() const {
    return newtons_;
}

double Force::asNewtons() const {
    return newtons_;
}

double Force::asMillinewtons() const {
    return newtons_ * 1'000.0;
}

double Force::asKilonewtons() const {
    return newtons_ / 1'000.0;
}

bool Force::approximatelyEquals(Force other, double epsilon) const {
    return detail::absolute(newtons_ - other.newtons_) <= epsilon;
}

QuantityDisplay<Force, ForceUnit> Force::displayAs(ForceUnit unit) const {
    return QuantityDisplay<Force, ForceUnit>(*this, unit);
}

QuantityDisplay<Force, ForceUnit> Force::displayAsPrecision(ForceUnit unit, int precision) const {
    return QuantityDisplay<Force, ForceUnit>(*this, unit, precision);
}

} // namespace units
